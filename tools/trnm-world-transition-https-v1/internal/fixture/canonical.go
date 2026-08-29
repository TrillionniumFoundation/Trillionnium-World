package fixture

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"math"
	"regexp"
	"sort"
	"strconv"
	"unicode/utf8"
)

const maxCanonicalDepth = 128

var canonicalIntegerPattern = regexp.MustCompile(`^-?(0|[1-9][0-9]*)$`)

// ParseCanonical accepts only the strict JSON profile used by
// trnm_world_transition_v1: object/array root, no insignificant whitespace,
// strictly ascending object keys, signed-i64 integers only, minimal escaping,
// no duplicate keys, and bounded nesting.
func ParseCanonical(raw []byte, maximumBytes int) (any, error) {
	if len(raw) == 0 || (maximumBytes >= 0 && len(raw) > maximumBytes) {
		return nil, errors.New("canonical JSON is empty or exceeds its byte limit")
	}
	if !utf8.Valid(raw) {
		return nil, errors.New("canonical JSON is not valid UTF-8")
	}
	decoder := json.NewDecoder(bytes.NewReader(raw))
	decoder.UseNumber()
	value, err := parseCanonicalValue(decoder, 0)
	if err != nil {
		return nil, err
	}
	if _, ok := value.(map[string]any); !ok {
		if _, ok := value.([]any); !ok {
			return nil, errors.New("canonical JSON root must be an object or array")
		}
	}
	var trailing any
	if err := decoder.Decode(&trailing); !errors.Is(err, io.EOF) {
		return nil, errors.New("canonical JSON has trailing data")
	}
	canonical, err := CanonicalJSON(value)
	if err != nil {
		return nil, err
	}
	if !bytes.Equal(canonical, raw) {
		return nil, errors.New("JSON bytes are not the exact canonical encoding")
	}
	return value, nil
}

func parseCanonicalValue(decoder *json.Decoder, depth int) (any, error) {
	if depth > maxCanonicalDepth {
		return nil, errors.New("canonical JSON nesting exceeds 128")
	}
	token, err := decoder.Token()
	if err != nil {
		return nil, fmt.Errorf("read canonical JSON token: %w", err)
	}
	switch typed := token.(type) {
	case json.Delim:
		switch typed {
		case '{':
			object := make(map[string]any)
			previous := ""
			first := true
			for decoder.More() {
				keyToken, keyErr := decoder.Token()
				if keyErr != nil {
					return nil, fmt.Errorf("read object key: %w", keyErr)
				}
				key, ok := keyToken.(string)
				if !ok {
					return nil, errors.New("object key is not a string")
				}
				if !first && key <= previous {
					return nil, errors.New("object keys are duplicated or not strictly ascending")
				}
				first = false
				previous = key
				child, childErr := parseCanonicalValue(decoder, depth+1)
				if childErr != nil {
					return nil, childErr
				}
				object[key] = child
			}
			closing, closeErr := decoder.Token()
			if closeErr != nil || closing != json.Delim('}') {
				return nil, errors.New("object is not closed")
			}
			return object, nil
		case '[':
			array := make([]any, 0)
			for decoder.More() {
				child, childErr := parseCanonicalValue(decoder, depth+1)
				if childErr != nil {
					return nil, childErr
				}
				array = append(array, child)
			}
			closing, closeErr := decoder.Token()
			if closeErr != nil || closing != json.Delim(']') {
				return nil, errors.New("array is not closed")
			}
			return array, nil
		default:
			return nil, errors.New("unexpected JSON delimiter")
		}
	case json.Number:
		text := typed.String()
		if text == "-0" || !canonicalIntegerPattern.MatchString(text) {
			return nil, errors.New("JSON numbers must be canonical signed-i64 integers")
		}
		value, parseErr := strconv.ParseInt(text, 10, 64)
		if parseErr != nil {
			return nil, errors.New("JSON integer exceeds signed-i64 range")
		}
		return value, nil
	case string, bool, nil:
		return typed, nil
	default:
		return nil, fmt.Errorf("unsupported canonical JSON scalar %T", token)
	}
}

// CanonicalJSON encodes values using the exact contract profile.
func CanonicalJSON(value any) ([]byte, error) {
	buffer := bytes.NewBuffer(nil)
	if err := appendCanonical(buffer, value, 0); err != nil {
		return nil, err
	}
	return buffer.Bytes(), nil
}

func appendCanonical(buffer *bytes.Buffer, value any, depth int) error {
	if depth > maxCanonicalDepth {
		return errors.New("canonical JSON nesting exceeds 128")
	}
	switch typed := value.(type) {
	case nil:
		buffer.WriteString("null")
	case bool:
		if typed {
			buffer.WriteString("true")
		} else {
			buffer.WriteString("false")
		}
	case string:
		appendJSONString(buffer, typed)
	case int:
		buffer.WriteString(strconv.FormatInt(int64(typed), 10))
	case int8:
		buffer.WriteString(strconv.FormatInt(int64(typed), 10))
	case int16:
		buffer.WriteString(strconv.FormatInt(int64(typed), 10))
	case int32:
		buffer.WriteString(strconv.FormatInt(int64(typed), 10))
	case int64:
		buffer.WriteString(strconv.FormatInt(typed, 10))
	case uint:
		if uint64(typed) > math.MaxInt64 {
			return errors.New("unsigned integer exceeds signed-i64 range")
		}
		buffer.WriteString(strconv.FormatUint(uint64(typed), 10))
	case uint8:
		buffer.WriteString(strconv.FormatUint(uint64(typed), 10))
	case uint16:
		buffer.WriteString(strconv.FormatUint(uint64(typed), 10))
	case uint32:
		buffer.WriteString(strconv.FormatUint(uint64(typed), 10))
	case uint64:
		if typed > math.MaxInt64 {
			return errors.New("unsigned integer exceeds signed-i64 range")
		}
		buffer.WriteString(strconv.FormatUint(typed, 10))
	case []any:
		buffer.WriteByte('[')
		for index, child := range typed {
			if index != 0 {
				buffer.WriteByte(',')
			}
			if err := appendCanonical(buffer, child, depth+1); err != nil {
				return err
			}
		}
		buffer.WriteByte(']')
	case map[string]any:
		keys := make([]string, 0, len(typed))
		for key := range typed {
			keys = append(keys, key)
		}
		sort.Strings(keys)
		buffer.WriteByte('{')
		for index, key := range keys {
			if index != 0 {
				buffer.WriteByte(',')
			}
			appendJSONString(buffer, key)
			buffer.WriteByte(':')
			if err := appendCanonical(buffer, typed[key], depth+1); err != nil {
				return err
			}
		}
		buffer.WriteByte('}')
	default:
		return fmt.Errorf("unsupported canonical JSON value %T", value)
	}
	return nil
}

func appendJSONString(buffer *bytes.Buffer, value string) {
	buffer.WriteByte('"')
	for _, character := range value {
		switch character {
		case '"':
			buffer.WriteString(`\"`)
		case '\\':
			buffer.WriteString(`\\`)
		case '\b':
			buffer.WriteString(`\b`)
		case '\f':
			buffer.WriteString(`\f`)
		case '\n':
			buffer.WriteString(`\n`)
		case '\r':
			buffer.WriteString(`\r`)
		case '\t':
			buffer.WriteString(`\t`)
		default:
			if character < 0x20 {
				buffer.WriteString(`\u00`)
				buffer.WriteByte("0123456789abcdef"[(character>>4)&0x0f])
				buffer.WriteByte("0123456789abcdef"[character&0x0f])
			} else {
				buffer.WriteRune(character)
			}
		}
	}
	buffer.WriteByte('"')
}

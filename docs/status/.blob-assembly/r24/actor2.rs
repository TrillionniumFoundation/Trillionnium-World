//B00
}

//B01
            ))
        } else {
//B02
            }
        };
//B03
}

//B04
        .await;
    });
//B05
        }
    }
//B06
                }
            }
//B07
                }
            }
//B08
                }
            }
//B09
                    break;
                }
                match derive_terminal_result(&loaded.simulation, &loaded.match_mode) {
                    Ok((_result, result_hash)) => {
                        state_sequence = state_sequence.saturating_add(1);
                        publication_candidates.send_replace(Some(ActorPublicationCandidate {
                            state: PublishedMatchState {
                                simulation: Arc::new(loaded.simulation.clone()),
                                snapshot_hash: Arc::new(snapshot_hash),
                                next_sequence: loaded.next_sequence,
                                match_revision: loaded.match_revision,
                                next_input_sequences: Arc::new(
                                    loaded.next_input_sequences.clone(),
                                ),
                                phase: OnlineMatchPhase::Complete,
                                result_hash: Some(result_hash),
                                // Internal-only until the fenced ACK transaction
                                // returns the committed settlement state.
                                settlement_state: "staged".to_string(),
                                state_sequence,
                                published_at: Instant::now(),
                            },
                            durable_db_next_sequence: loaded.next_sequence,
                            durable_db_match_revision: loaded.match_revision,
                            durable_db_next_input_sequences: loaded
                                .next_input_sequences
                                .clone(),
                            receipts_replayable: true,
                            publish_to_watch: true,
                        }));
                        terminal_publication_sequence = Some(state_sequence);
                        if let Some(pending) = pending_terminal_receipt.as_mut() {
                            pending.state_sequence = state_sequence;
                        }
                    }
                    Err(error) => {
                        tracing::error!(
                            %match_id,
                            %error,
                            "terminal stage committed but its deterministic result could not be derived"
                        );
                        break;
                    }
                }
            }
            completion = publication_completion_rx.recv() => {
                let Some(completion) = completion else {
                    if let Some(pending) = pending_published_receipt.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "published-tick journal stopped before command acknowledgement",
                            true,
                        )));
                    }
                    if let Some(pending) = pending_terminal_receipt.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "published-tick journal stopped before terminal acknowledgement",
                            true,
                        )));
                    }
                    tracing::error!(%match_id, "published-tick journal worker stopped unexpectedly");
                    break;
                };
                let ActorPublicationCompletion {
                    state_sequence: completed_sequence,
                    deferred_terminal,
                    result,
                } = completion;
                if let Err(error) = result {
                    if let Some(pending) = pending_published_receipt.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            format!("durable publication failed: {error}"),
                            true,
                        )));
                    }
                    if let Some(pending) = pending_terminal_receipt.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            format!("terminal durable publication failed: {error}"),
                            true,
                        )));
                    }
                    tracing::error!(%match_id, %error, "published-tick durability failed closed");
                    break;
                }
//B0B
                    continue;
                }
//B0C
                    }
                };
//B0D
                break;
            }
//B0E
                        }
                    }
//B0F
                            break;
                        }
//B10
                            }
                        };
//B11
                        }
                    }
//B12
                }
            }
//B13
                    }
                }
//B14
                    continue;
                }
//B15
                    }
                };
//B16
        }
    }
//B17

            sql.contains("old.phase = 'running' and new.phase = 'failed_closed' and not exists")
            "raise exception 'terminal publication ACK settlement state cannot regress or change'"
            Some(("0001_online_authority_v1", &"0".repeat(64))),

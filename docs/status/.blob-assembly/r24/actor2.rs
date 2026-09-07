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
            completion = terminal_checkpoint_completion_rx.recv(),
                if terminal_checkpoint_handle.is_some() => {
                terminal_checkpoint_handle.take();
                let Some(TerminalCheckpointCompletion { snapshot_hash, result }) = completion else {
                    tracing::error!(%match_id, "terminal checkpoint task stopped before acknowledgement");
                    break;
                };
                if let Err(error) = result {
                    tracing::error!(%match_id, %error, "terminal actor checkpoint failed closed");
                    break;
                }
//B0A
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
            completion = persistence_completion_rx.recv(), if pending_command.is_some() => {
                let Some(completion) = completion else {
                    if let Some(pending) = pending_command.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "command persistence worker stopped before acknowledgement",
                            true,
                        )));
                    }
                    tracing::error!(%match_id, "match actor command persistence worker stopped unexpectedly");
                    break;
                };
                let Some(pending) = pending_command.take() else {
                    tracing::error!(%match_id, "command persistence completed without an actor barrier");
                    break;
                };
                if completion.command_id != pending.command_id {
                    let _ = pending.response.send(Err(api_error(
                        StatusCode::SERVICE_UNAVAILABLE,
                        "command persistence completion did not match the actor barrier",
                        true,
                    )));
                    tracing::error!(
                        %match_id,
                        expected_command_id = %pending.command_id,
                        completed_command_id = %completion.command_id,
                        "match actor command persistence ordering failed closed"
                    );
                    break;
                }
                match completion.result {
                    Ok(receipt) if receipt.duplicate => {
                        debug_assert!(!actor_command_lane_publish_allowed(true, false));
                        let recovery_target_tick = loaded
                            .simulation
                            .tick
                            .max(latest_visible_candidate_tick);
                        let recovery = tokio::time::timeout(
                            MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
                            reload_visible_actor_after_persistence_failure(
                                &state,
                                match_id,
                                pending.visible,
                                recovery_target_tick,
                                latest_visible_candidate_tick,
                            ),
                        )
                        .await
                        .map_err(|_| "duplicate command recovery timed out".to_string())
                        .and_then(|result| result);
                        match recovery {
                            Ok(reloaded) => {
                                loaded = reloaded;
                                let recovered_at = Instant::now();
                                clock.reset(recovered_at);
                                interval.reset_at(recovered_at + state.tick_interval);
                                cache_actor_receipt(
                                    &mut receipt_cache,
                                    &mut receipt_cache_order,
                                    pending.request_hash,
                                    &receipt,
                                );
                                let _ = pending.response.send(Ok(receipt));
                            }
                            Err(error) => {
                                let _ = pending.response.send(Err(api_error(
                                    StatusCode::SERVICE_UNAVAILABLE,
                                    format!("duplicate command recovery failed: {error}"),
                                    true,
                                )));
                                tracing::error!(%match_id, %error, "duplicate command race failed closed");
                                break;
                            }
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

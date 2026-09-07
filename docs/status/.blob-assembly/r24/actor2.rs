async fn compact_published_tick_journal_with_timeout(
    journal: &PublishedTickJournal,
    active_matches: BTreeSet<Uuid>,
) -> Result<(), String> {
    let mut guard = JournalOperationGuard::new(journal);
    let result = match tokio::time::timeout(
        MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
        journal.compact_to_active(active_matches),
    )
    .await
    {
        Ok(result) => result,
        Err(_) => {
            journal.fail_closed();
            Err(
                "published-tick compaction exceeded its hard timeout and is failed closed"
                    .to_string(),
            )
        }
    };
    guard.complete();
    result
}

struct JournalOperationGuard<'a> {
    journal: &'a PublishedTickJournal,
    completed: bool,
}

impl<'a> JournalOperationGuard<'a> {
    fn new(journal: &'a PublishedTickJournal) -> Self {
        Self {
            journal,
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}

impl Drop for JournalOperationGuard<'_> {
    fn drop(&mut self) {
        if !self.completed {
            self.journal.fail_closed();
        }
    }
}

fn release_deferred_terminal_publication(
    deferred: DeferredTerminalPublication,
    published: &watch::Sender<PublishedMatchState>,
    publication_acked: &watch::Sender<ActorPublicationCursor>,
) -> Result<(), String> {
    if deferred.state.phase != OnlineMatchPhase::Complete
        || deferred.cursor.phase != OnlineMatchPhase::Complete
        || !publication_tuple_matches(&deferred.state, &deferred.cursor)
        || TerminalPublicationEvidence::from_state(&deferred.state).as_ref()
            != Some(&deferred.evidence)
    {
        return Err(
            "deferred terminal publication tuple changed before marker acknowledgement".to_string(),
        );
    }
    if deferred.publish_to_watch {
        published.send_replace(deferred.state);
    }
    publication_acked.send_replace(deferred.cursor);
    Ok(())
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

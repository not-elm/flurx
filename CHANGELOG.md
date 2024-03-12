## 0.1.2

### Futures

- Add `Scheduler::run_sync`. To use it, add the `sync` flag to features.

### Refactor

- Rename `TaskCreator` to `ReactiveTask`, and rename `TaskCreator::task` to `ReactiveTask::will`
- Delete `wait::while_` 

### Bug Fix

- Fixed wrong exit condition for `wait::until` (changed to wait until it returns true)


## 0.1.1

- Fix documents.
# profiling

## flame

Flame is just a wall clock timer library that leverages the implicit call to `Drop` at the end of scope in order to serialize wall clock profiling information about the running process.

It falls under the non-preemptive style of profiling, as there are known points where the profiler is allowed to run. These are the calls to `flame::start_guard` and the implicit call to drop at the end of scope for the guard object.

While Flame can be used in a multi-threaded environment, it only tracks information about a single thread in the final html output.

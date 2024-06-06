## Futures, Tasks, and Threads

<!-- TODO: discuss tasks vs. threads more *here*? -->

As we saw in the previous chapter, threads provide one approach to concurrency,
and they let us solve some of these issues. However, they also have some
tradeoffs. On many operating systems, they use a fair bit of memory for each
thread, and they come with some overhead for starting up and shutting down.
Threads are also only an option when your operating system and hardware support
them! While mainstream desktop and mobile operating systems have all had
threading for many years, many embedded operating systems, like those used on
some microcontrollers, do not.

The async model provides a different—and ultimately complementary—set of
tradeoffs. In the async model, concurrent operations do not require their own
threads. Instead, they can run on *tasks*. A task is a bit like a thread, but
instead of being managed by the operating system, it is managed by code that
lives at the level of libraries.

<!--
  TODO: connective tissue as it were. Also, an open question on whether we want
  to use “task” as the primary term here. Futures do not actually *require* a
  task primitive to run (and there are runtimes which do *not* use tasks!) so it
  might make more sense to find another common, appropriate term for it instead.
-->

### Parallelism and Concurrency

First, though, we need to dig a little deeper into the differences between
parallelism and concurrency. In the previous chapter we treated them as mostly
interchangeable. Now we need to distinguish between the two a little more,
because the differences will show up as we start working:

* *Parallelism* is when operations can happen simultaneously.

* *Concurrency* is when operations can make progress without having to wait for
  all other operations to complete.

One common analogy for thinking about the difference between concurrency and
parallelism is cooking in a kitchen. Parallelism is like having two cooks: one
working on cooking eggs, and the other working on preparing fruit bowls. Those
can happen at the same time, without either affecting the other. Concurrency is
like having a single cook who can start cooking some eggs, start dicing up some
vegetables to use in the omelette, adding seasoning and whatever vegetables are
ready to the eggs at certain points, and switching back and forth between those
tasks.

(This analogy breaks down if you think about it too hard. The eggs keep cooking
while the cook is chopping up the vegetables, after all. That is parallelism,
not just concurrency! The focus of the analogy is the *cook*, not the food,
though, and as long as you keep that in mind, it mostly works.)

On a machine with multiple CPU cores, we can actually do work in parallel. One
core can be doing one thing while another core does something completely
unrelated, and those actually happen at the same time. On a machine with a
single CPU core, the CPU can only do one operation at a time, but we can still
have concurrency. Using tools like threads, processes, and async, the computer
can pause one activity and switch to others before eventually cycling back to
that first activity again. So all parallel operations are also concurrent, but
not all concurrent operations happen in parallel!

When working with async in Rust, we are always dealing with concurrency.
Depending on the hardware, the operating system, and the async runtime we are
using, that concurrency may use some degree of parallelism under the hood, or it
may not. (More about async runtimes later!)

A big difference between the cooking analogy and Rust’s async model for
concurrency is that in the cooking example, the cook makes the decision about
when to switch tasks. In Rust’s async model, the tasks are in control of that.
To see how, let’s look at how Rust actually uses async.

## Misc. other stuff

<!-- TODO: find this a home or scrap it -->
The `async` keyword does not yet work with closures directly. That is, there is
no direct equivalent to `async fn` for anonymous functions. As a result, you
cannot write code like these function calls:

```rust,ignore
example_1(async || { ... });
example_2(async move || { ... });
```

However, since async blocks themselves can be marked with `move`, this ends up
not being a problem. Because `async` blocks compile to anonymous futures, you
can write calls like this instead:

```rust,ignore
example_1(|| async { ... });
example_2(|| async move { ... });
```

These closures now return anonymous futures, meaning they work basically the
same way that an async function does.
<!-- TODO: through here -->

<!-- TODO: find this a home or scrap it -->
> Note: This is how closures work, too, but we did not have to talk about it
> back in Chapter 13, because the details did not bubble up to the surface the
> way they do here!
<!-- TODO: through here -->

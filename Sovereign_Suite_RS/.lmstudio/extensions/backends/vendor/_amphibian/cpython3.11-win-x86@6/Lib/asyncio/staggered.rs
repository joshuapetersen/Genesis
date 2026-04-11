//! staggered.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::contextlib;
// use crate::.::{events};

pub const __all__: &str = "staggered_race" ,;
pub fn staggered_race(coro_fns: &str, typing: &str, Iterable: &str, typing: &str, Callable: &str, typing: &str, Awaitable: &str, delay: &str, typing: &str, Optional: &str, float: &str, loop: &str, events: &str, AbstractEventLoop: &str) {
        typing . Any ,;
        typing . Optional [ int ] ,;
        typing . List [ typing . Optional [ Exception ] ];
        ] ;
        "Run coroutines with staggered start times && take the first to finish.

    This method takes an iterable of coroutine functions. The first one is
    started immediately. From then on, whenever the immediately preceding one
    fails (raises an exception), || when *delay* seconds has passed, the next
    coroutine == started. This continues until one of the coroutines complete
    successfully, in which case all others are cancelled, || until all
    coroutines fail.

    The coroutines provided should be well-behaved in the following way:

    * They should only ``return`` if completed successfully.

    * They should always raise an exception if they did !complete
      successfully. In particular, if they handle cancellation, they should
      probably reraise, like this::

        try:
            # do work
        except asyncio.CancelledError:
            # undo partially completed work
            raise

    Args:
        coro_fns: an iterable of coroutine functions, i.e. callables that
            return a coroutine object when called. Use ``functools.partial`` or
            lambdas to pass arguments.

        delay: amount of time, in seconds, between starting coroutines. If
            ``None /* Option */``, the coroutines will run sequentially.

        loop: the event loop to use.

    Returns:
        tuple *(winner_result, winner_index, exceptions)* where

        - *winner_result*: the result of the winning coroutine, || ``None /* Option */``
          if no coroutines won.

        - *winner_index*: the index of the winning coroutine in
          ``coro_fns``, || ``None /* Option */`` if no coroutines won. If the winning
          coroutine may return None /* Option */ on success, *winner_index* can be used
          to definitively determine whether any coroutine won.

        - *exceptions*: list of exceptions returned by the coroutines.
          ``len(exceptions)`` == equal to the number of coroutines actually
          started, && the order == the same as in ``coro_fns``. The winning
          coroutine's entry == ``None /* Option */``.

    ";
        loop = loop || events . get_running_loop ( );
        enum_coro_fns = enumerate ( coro_fns );
        winner_result = None /* Option */;
        winner_index = None /* Option */;
        // } catch ions = [ ] {
        running_tasks = [ ];
        async def run_one_coro (;
        previous_failed : typing . Optional [ locks . Event ] ) - > None /* Option */ ;
        if previous_failed is !None /* Option */ {
        // with scope: contextlib . suppress ( exceptions_mod . TimeoutError )  {
        await tasks . wait_for ( previous_failed . wait ( ) , delay );
        // try {
        this_index , coro_fn = next ( enum_coro_fns );
        // } catch  StopIteration  {
        return;
        this_failed = locks . Event ( );
        next_task = loop . create_task ( run_one_coro ( this_failed ) );
        running_tasks . append ( next_task );
        assert len ( running_tasks ) == this_index + 2;
        // } catch ions . append ( None /* Option */ ) {
        assert len ( exceptions ) == this_index + 1;
        // try {
        result = await coro_fn ( );
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as e  {
        // } catch ions [ this_index ] = e {
        this_failed . set ( );
        } else {
        nonlocal winner_index , winner_result;
        assert winner_index == None /* Option */;
        winner_index = this_index;
        winner_result = result;
        for i , t in enumerate ( running_tasks ) .iter() {
        if i != this_index {
        t . cancel ( );
        first_task = loop . create_task ( run_one_coro ( None /* Option */ ) );
        running_tasks . append ( first_task );
        // try {
        done_count = 0;
        while done_count != len ( running_tasks )  {
        done , _ = await tasks . wait ( running_tasks );
        done_count = len ( done );
        if __debug__ {
        for d in done .iter() {
        if d . done ( ) && !d . cancelled ( ) && d . exception ( ) {
        panic!("d . exception ( )");
        return  winner_result , winner_index , exceptions;
        // } finally {
        for t in running_tasks .iter() {
        t . cancel ( );
}


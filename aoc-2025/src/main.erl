-module(main).
-behaviour(application).

-export([start/2, stop/1]).

start(_StartType, _StartArgs) ->
    io:format("Hello World!~n"),
    {ok, spawn(fun() -> loop() end)}.

stop(_State) ->
    ok.

% Simple loop to keep the process alive
loop() ->
    receive
        stop -> ok
    end.

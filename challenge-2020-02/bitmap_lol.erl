-module(bitmap_lol).

-export([
    new/1,
    get/2,
    set/2,
    clear/2,
    len/1
]).

new(NumEntries) ->
    ["NO" || _ <- lists:seq(1, NumEntries)].

get([], _) -> error;
get([X | _], 0) -> {ok, X};
get([_ | R], N) -> get(R, N-1).

set([], _) -> error;
set([X | R], 0) -> {ok, X, ["YES" | R]};
set([X | R], N) ->
    case set(R, N-1) of
        error -> error;
        {ok, Y, L} -> {ok, Y, [X | L]}
    end.

clear([], _) -> error;
clear([X | R], 0) -> {ok, X, ["NO" | R]};
clear([X | R], N) ->
    case clear(R, N-1) of
        error -> error;
        {ok, Y, L} -> {ok, Y, [X | L]}
    end.

len(L) ->
    length([1 || X <- L, X =:= "YES"]).

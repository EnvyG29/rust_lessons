sum([], 0).
sum([Head|Tail], Total):-
    sum(Tail, SumCurrent), Total is Head + SumCurrent.
	    
?- sum([1,2,3,4,5,6,7,8,9], X), write(X); write("¯\\_(ツ)_/¯").

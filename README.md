# My [Infi Advent of Code (2017)](https://aoc.infi.nl/) solution

The challenge (text in Dutch):

> ## Welkom bij de Advent of Code puzzel van Infi!
> 
> Robots wil eat your job!
> Bij Infi houden wij niet van repetitief werk en dus proberen we dit jaar wat tijdrovende taken te automatiseren.
> Voor het uitdelen van de kerstcadeaus hebben wij daarom bezorgrobots gebouwd, zodat wij ons volledig kunnen richten op code committen en kerstborrels bezoeken.
> Helaas zitten we pas op release 0.9, want we kwamen er achter dat soms meerdere robots op dezelfde plek uit kunnen komen en dat is natuurlijk niet efficiënt.
> We moeten dit snel oplossen door te bepalen hoe vaak deze situatie voorkomt, want het is al bijna de 25ste!
> Help jij mee?
> Om te helpen met debuggen hebben we enkele logs beschikbaar gemaakt. Deze zijn in het volgende formaat opgeslagen:
>
>     [sx1,sy1][sx2,sy2](x1,y1)(x2,y2)(x1,y1)
>
> Eerst vind je tussen de blokhaken de startposities van de robots.
> Let op: schaalbaarheid is belangrijk, dus het aantal robots is variabel!
> Vervolgens bevat het log de bewegingen die door de robots uitgevoerd zijn, in dezelfde volgorde als dat de robots zijn gedefinieerd.
> 
> Voorbeeld:
> 
>     [0,0][1,1](1,0)(0,-1)(0,1)(-1,0)(-1,0)(0,1)(0,-1)(1,0)
> 
> 1.  Robot 1 begint op 0,0 en Robot 2 begint op 1,1
> 2.  Robot 1 gaat naar 1,0 (0,0 + 1,0)
> 3.  Robot 2 gaat naar 1,0 (1,1 + 0,-1)
> 4.  OEPS! Dit is dus een knelpunt.
> 5.  Robot 1 gaat naar 1,1 (1,0 + 0,1)
> 6.  Robot 2 gaat naar 0,0 (1,0 + -1,0)
> 7.  Robot 1 gaat naar 0,1 (1,1 + -1,0)
> 8.  Robot 2 gaat naar 0,1 (0,0 + 0,1)
> 9.  AI, Dit is ook een knelpunt.
> 10. Robot 1 gaat naar 0,0 (0,1 + 0,-1)
> 11. Robot 2 gaat naar 1,1 (0,1 + 1,0)
> 
> Het komt in dit voorbeeld dus 2 keer voor dat de robots elkaar tegen komen.
> Kun jij uitrekenen hoe vaak dit is gebeurd voor het volgende logbestand?

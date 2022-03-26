jestem botem do koblowania w silniku MIRA. komendy muszą rozpoczynać się sekwencją `:kobler` lub `:k`. dostępne są komendy:

```
kurwa — wyświetla wsparcie.

prawzór (argument) — użyta bez argumentu wyświetla prawzór wybranego gracza, lub informuje o jego braku. użyta z argumentem ustawia prawzór gracza na wybrany. dostępne argumenty to:
  w, wojownik — ustawia prawzór wojownika.
  z, złodziej — ustawia prawzór złodzieja.
  k, kapłan — ustawia prawzór kapłana.

rzut — wykonuje rzut kośćmi, umożlwiwia wybranie stopnia wyszkolenia oraz narzędzia albo broni. aby wykonać rzut, gracz musi mieć ustawiony prawzór. dostępne argumenty to:
  -s — ustawia wyszkolenie podstawowe.
  -sb — ustawia wyszkolenie biegłe.
  -b WZG — ustawia broń. znak W reprezentuje wagę spośród (L lekka, C ciężka). znak Z reprezentuje zasieg spośród (B biała, D dalekosiężna). znak G reprezentuje grupę spośród (A agilna, B brutalna, C cwana).
  -n JG — ustawia narzędzie. znak J reprezentuje jakość spośród (M morowe, P przyzwoite, L liche). znak G reprezentuje grupę spośród (A agilna, B brutalna, C cwana).

zanik - wykonuje próbę zaniku, wymaga podania aktualnej trwałości sprzętu jako pierwszego argumentu. umożliwia zmianę jakości sprzętu (domyślnie przyzwoita):
  -M — ustawia jakość morową.
  -P — ustawia jakość przyzwoitą.
  -L — ustawia jakość lichą.
```

przykładowo:

```
:kobler prawzór kapłanka — ustawia użytkowiniczce prawzór kapłanki.
:kobler rzut -s -n PB — wykonuje rzut z wyszkoleniem podstawowym oraz przyzwoitym brutalnym narzędziem.
:kobler rzut -b CBA — wykonuje rzut z ciężką białą bronią agilną.
:kobler zanik 2 -M — wykonuje rzut zaniku dla morowego narzędzia o trwałości dwa.
```

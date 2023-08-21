jestem botem do koblowania zgodnego ze Zdrżeniem Urojonych Gier. komendy muszą rozpoczynać się sekwencją `:kobler` lub `:k`. dostępne są komendy:

```
kurwa : wyświetla wsparcie.


wzorzec : użyta bez argumentu wyświetla wzorzec wybranego gracza, lub informuje o jego braku. użyta z argumentem ustawia wzorzec gracza na wybrany. dostępne argumenty to:
  -c, --chojrak, --chojraczka — ustawia wzorzec chojraka.
  -s, --szelma — ustawia wzorzec szelmy.


broń : użyta bez argumentu wyświetla bronie wybranego gracza. dostępne argumenty to:
  dodaj : dodaje nową broń gracza (jako niewybraną). konieczne opcje to:
    -n STR, --nazwa STR: ustawia identyfikator broni. znaki STR reprezentują ciąg znaków bez białych lub specjalnych innych niż '_'.
    -w W, --waga W : ustawia wagę broni. znak W reprezentuje wagę spośród (L lekka, C ciężka).
    -z Z, --zasięg Z : ustawia zasięg broni. znak Z reprezentuje zasięg spośród (B biała, M Miotająca).
  wybierz : wybiera wskazaną broń jako aktywną. konieczny argument to:
    -n STR, --nazwa STR : identyfikator wybranej borni.
  schowaj : wybiera wskazaną broń jako nieaktywną. konieczny argument to:
    -n STR, --nazwa STR : identyfikator wybranej borni.
  usuń : usuwa wskazaną broń z wyposażenia gracza. konieczny argument to:
    -n STR, --nazwa STR : identyfikator wybranej borni.
  

próba : wykonuje rzut kośćmi jak przy próbie, umożlwiwia wybranie stopnia przeszkolenia oraz narzędzia. aby wykonać próbę, gracz musi mieć ustawiony wzorzec. dostępne argumenty to:
  -s, --szkolony : ustawia przeszkolenie podstawowe.
  -b, --biegły : ustawia przeszkolenie biegłe.
  -z, --znakomita : ustawia jakość znakomitą narzędzia.
  -p, --przyzwoita : ustawia jakość przyzwoitą narzędzia.
  -k, --kiepska : ustawia jakość lichą narzędzia.


bitwa : wykonuje rzut kośćmi jak przy bitwie. aby wykonać rzut, gracz musi mieć ustawiony wzorzec. korzysta z wszystkich wybranych broni gracza. dostępne argumenty to:
  -p N, --plus N : zwiększa liczbę kości wzorca przy rzucie o wskazaną liczbę N.
  -m N, --minus N : zmniejsza liczbę kości wzorca przy rzucie o wskazaną liczbę N.


zanik : wykonuje próbę zaniku, wymaga podania aktualnej wytrzymałości sprzętu jako pierwszego argumentu. umożliwia zmianę jakości sprzętu (domyślnie przyzwoita, jeśli zostanie podana więcej niż jedna, pod uwagę wzięta zostanie tylko ostatnia):
  -z, --znakomita : ustawia jakość znakomitą.
  -p, --przyzwoita : ustawia jakość przyzwoitą.
  -k, --kiepska : ustawia jakość lichą.
```

przykładowo:

```
:kobler wzorzec --chojraczka : ustawia użytkowiniczce wzorzec chojraczki.
:kobler broń dodaj -n rozkurwiator -wC --zasięg B : ustawia użytkowkikowi białą broń cieżką o nazwie 'rozkurwiator'.
:kobler próba -s -k : wykonuje rzut z wyszkoleniem podstawowym oraz kiepskim narzędziem.
:kobler bitwa -m 1 : wykonuje rzut trzema kośćmi wzorca i aktywnymi brońmi gracza.
:kobler zanik 2 -z : wykonuje rzut zaniku dla znakomitego narzędzia o trwałości dwa.
```

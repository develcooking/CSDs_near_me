# CSDs_near_me

## Was ist das?
Dieses Programm ruft bevorstehende CSD (Christopher Street Day) Veranstaltungen in der Nähe einer angegebenen Stadt ab und berechnet die Entfernungen zu diesen Veranstaltungen von der Stadt des Benutzers

## Warum?
Auf der Website [CSD-Termine.de](https://www.csd-termine.de/) werden alle CSD's für ein Jahr angezeigt. Da dies aber sehr unübersichtlich ist habe ich dieses kleine Projekt gebastelt.
Ich habe dieses Prgramm uhrsprünglich in Python programmiert um damit a weng zu experimentieren. Da das Prgramm dadurch aber sehr langsam wahr bin ich wieder zurück zu rust gesprungen. Dies ein kleines side Projekt von mir wird vermutlich keine großen updates mehr bekommen.   
Mit den API key dürft Ihr anstellen, was ihr möchtet 

## Verwendung
1. Führen Sie das Programm aus.
2. Geben Sie den Namen Ihrer Stadt ein, wenn Sie dazu aufgefordert werden.
3. Das Programm ruft bevorstehende CSD-Veranstaltungsorte und -daten in der Nähe Ihrer Stadt ab.
4. Es berechnet die Entfernungen zwischen diesen Veranstaltungsorten und Ihrer Stadt.
5. Schließlich zeigt es die sortierte Liste von Städten mit ihren Entfernungen von Ihrer Stadt an.

## Installation
1. Klonen Sie das Repository.
```git clone https://github.com/develcooking/CSDs_near_me```
2. Installieren Sie die [erforderlichen](https://rustup.rs/) Abhängigkeiten. 
3. Kompilieren und führen Sie das Programm aus.
```cargo run```

## Wie funktioniert das?
Es verwendet Web-Scraping, um Veranstaltungsorte und -daten von einer bestimmten URL zu extrahieren, und Geocoding, um die Koordinaten von Städten für die Entfernungsberechnung zu bestimmen.

Abhänigkeiten, die dafür im Hintergrund genutzt werden (werden automatisch installiert)
- reqwest
- select
- chrono
- geo
- geocoding

## Lizenz
Dieses Programm steht unter der [MIT-Lizenz](https://opensource.org/licenses/MIT).
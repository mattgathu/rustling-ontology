use super::*;
use rustling_ontology_values::check::*;

pub fn examples_temperature(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_temperature(10.0, Some("degree")), "10 Grad");
    example!(v, check_temperature(-20.0, None), "minus 20");
    example!(v, check_temperature(-20.0, Some("degree")), "minus 20 Grad");
    example!(v, check_temperature(3.0, Some("degree")), "plus 3 Grad");
    example!(v, check_temperature(71.0, Some("degree")), "71 Grad");
    example!(v, check_temperature(-7.0, Some("degree")), "sieben Grad unter null");
    example!(v, check_temperature(-10.0, None), "10 unterm gefrierpunkt");
    example!(v, check_temperature(5.0, Some("degree")), "5 grad über null");
    example!(v, check_temperature(20.0, Some("degree")), "20 grad über null");
    example!(v, check_temperature(15.0, Some("celsius")), "15 C", "15°C");
    example!(v, check_temperature(23.0, Some("degree")), "plus 23 °", "23°");
    example!(v, check_temperature(-24.0, Some("degree")), "minus 24 grad");
    example!(v, check_temperature(-15.0, Some("degree")), "15 grad unter den gefrierpunkt");
    example!(v, check_temperature(-34.0, Some("degree")), "34 grad unterm gefrierpunkt");
    example!(v, check_temperature(13.0, Some("degree")), "plus 13 grad");
    example!(v, check_temperature(130.0, Some("kelvin")), "130 kelvin");
    example!(v, check_temperature(223.0, Some("kelvin")), "223 k");
    example!(v, check_temperature(78.0, Some("fahrenheit")), "78 fahrenheit", "78 f");
    example!(v, check_temperature(19.0, Some("degree")), "19 grad über null");
    example!(v, check_temperature(-18.0, Some("degree")), "Bei -18 Grad");
}

pub fn examples_finance(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_finance(30.0, Some("EUR"), Precision::Exact), "dreissig Euro");
    example!(v, check_finance(800.0, Some("$"), Precision::Exact), "800 dollar");
    example!(v, check_finance(800.0, Some("cent"), Precision::Exact), "achthundert cent");
    example!(v, check_finance(300.0, Some("cent"), Precision::Exact), "300 pennies");
    example!(v, check_finance(1.0, Some("cent"), Precision::Exact), "1 penny", "1 cent", "1 cts", "1 ct");
    example!(v, check_finance(4000.0, Some("INR"), Precision::Exact), "exakt viertausend rupien");
    example!(v, check_finance(980.0, Some("KRW"), Precision::Exact), "ganz genau neunhundertachtzig Won");
    example!(v, check_finance(478.0, Some("USD"), Precision::Exact), "478 US-Dollar");
    example!(v, check_finance(2134.0, Some("$"), Precision::Exact), "2134 $");
    example!(v, check_finance(9840.0, Some("£"), Precision::Exact), "9840 £");
    example!(v, check_finance(902.0, Some("£"), Precision::Approximate), "fast 902 Pfd.");
    example!(v, check_finance(849.0, Some("EUR"), Precision::Approximate), "ungefähr 849 €");
    example!(v, check_finance(4775.0, Some("EUR"), Precision::Exact), "haargenau 4775 Euro");
    example!(v, check_finance(90.0, Some("$"), Precision::Exact), "präzise neunzig $");
    example!(v, check_finance(674.0, Some("AUD"), Precision::Exact), "674 australische dollar");
    example!(v, check_finance(7438.0, Some("AUD"), Precision::Exact), "7438 AUD");
    example!(v, check_finance(6739.0, Some("EUR"), Precision::Approximate), "cirka 6739 €");
    example!(v, check_finance(839.0, Some("EUR"), Precision::Approximate), "ca 839 €");
    example!(v, check_finance(293.0, Some("EUR"), Precision::Approximate), "zirka 293 €");
    example!(v, check_finance(230983.0, Some("£"), Precision::Approximate), "beinahe 230983 £");
    example!(v, check_finance(150.0, Some("EUR"), Precision::Approximate), "nahezu hundertfünfzig Euro");
    example!(v, check_finance(100.0, Some("INR"), Precision::Exact), "sehr genau hundert indische Rupien");
}

pub fn examples_time(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    let c = ResolverContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second));
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 0]), "jetzt", "genau jetzt", "gerade eben");
    example!(v, check_moment!(c, [2013, 2, 12]), "heute", "zu dieser zeit");
    example!(v, check_moment!(c, [2013, 2, 11]), "gestern");
    example!(v, check_moment!(c, [2013, 2, 13]), "morgen");
    example!(v, check_moment!(c, [2013, 2, 18]), "montag", "mo.", "diesen montag");
    example!(v, check_moment!(c, [2013, 2, 18]), "Montag, Feb 18", "Montag, Februar 18");
    example!(v, check_moment!(c, [2013, 2, 19]), "dienstag");
    example!(v, check_moment!(c, [2013, 2, 14]), "donnerstag", "do", "do.");
    example!(v, check_moment!(c, [2013, 2, 15]), "freitag", "fr.");
    example!(v, check_moment!(c, [2013, 2, 16]), "samstag", "sa.");
    example!(v, check_moment!(c, [2013, 2, 17]), "sonntag", "so.");
    example!(v, check_moment!(c, [2013, 3, 1]), "1 märz", "erster märz");
    example!(v, check_moment!(c, [2013, 3, 3]), "märz 3");
    example!(v, check_moment!(c, [2015, 3, 3]), "märz 3 2015");
    example!(v, check_moment!(c, [2013, 2, 15]), "am 15ten");
    example!(v, check_moment!(c, [2013, 2, 15]), "15. februar", "februar 15", "15te februar", "15.2.", "am 15.2.", "februar 15");
    example!(v, check_moment!(c, [2013, 8, 8]), "Aug 8");
    example!(v, check_moment!(c, [2014, 10]), "Oktober 2014");
    example!(v, check_moment!(c, [1974, 10, 31]), "31.10.1974", "31.10.74");
    example!(v, check_moment!(c, [2015, 4, 14]), "14 april 2015", "April 14, 2015", "14te April 15");
    example!(v, check_moment!(c, [2013, 2, 19]), "nächsten dienstag");
    example!(v, check_moment!(c, [2013, 2, 22]), "übernächsten freitag");
    example!(v, check_moment!(c, [2013, 3]), "nächsten marz");
    example!(v, check_moment!(c, [2014, 3]), "übernächsten marz");
    example!(v, check_moment!(c, [2013, 2, 10]), "Sonntag, Feb 10");
    example!(v, check_moment!(c, [2013, 2, 13]), "Mittwoch, Feb 13");
    example!(v, check_moment!(c, [2013, 2, 18]), "Montag, Feb 18");
    example!(v, check_moment!(c, [2013, 2, 11], Grain::Week), "diese woche");
    example!(v, check_moment!(c, [2013, 2, 18], Grain::Week), "kommende woche");
    example!(v, check_moment!(c, [2013, 2, 4], Grain::Week), "letzte woche");
    example!(v, check_moment!(c, [2013, 2, 18], Grain::Week), "nächste woche");
    example!(v, check_moment!(c, [2013, 1]), "letzten monat");
    example!(v, check_moment!(c, [2013, 3]), "nächsten monat");
    example!(v, check_moment!(c, [2013, 1, 1], Grain::Quarter), "dieses quartal");
    example!(v, check_moment!(c, [2013, 4, 1], Grain::Quarter), "nachstes quartal");
    example!(v, check_moment!(c, [2013, 7, 1], Grain::Quarter), "drittes quartal");
    example!(v, check_moment!(c, [2018, 10, 1], Grain::Quarter), "4tes quartal 2018");
    example!(v, check_moment!(c, [2012]), "letztes jahr");
    example!(v, check_moment!(c, [2013]), "dieses jahr");
    example!(v, check_moment!(c, [2014]), "nächstes jahr");
    example!(v, check_moment!(c, [2013, 2, 10]), "letzten sonntag", "sonntag der letzten woche", "sonntag letzte woche");
    example!(v, check_moment!(c, [2013, 2, 5]), "letzten dienstag");
    example!(v, check_moment!(c, [2013, 2, 19]), "nächsten dienstag");// when today is Tuesday, "mardi prochain" is a week from now
    example!(v, check_moment!(c, [2013, 2, 13]), "nächsten mittwoch");// when today is Tuesday, "mercredi prochain" is tomorrow
    example!(v, check_moment!(c, [2013, 2, 20]), "mittwoch der nachsten woche", "mittwoch nachste woche", "mittwoch nach dem nachsten"); // "mittwoch der nächsten woche", "mittwoch nächste woche", "mittwoch nach dem nächsten"
    example!(v, check_moment!(c, [2013, 2, 22]), "freitag nach dem nächsten");
    example!(v, check_moment!(c, [2013, 2, 11]), "montag dieser woche");
    example!(v, check_moment!(c, [2013, 2, 12]), "dienstag dieser woche");
    example!(v, check_moment!(c, [2013, 2, 13]), "mittwoch dieser woche");
    example!(v, check_moment!(c, [2013, 2, 14]), "übermorgen");
    example!(v, check_moment!(c, [2013, 2, 10]), "vorgestern");
    example!(v, check_moment!(c, [2013, 3, 25]), "letzter montag im märz");
    example!(v, check_moment!(c, [2014, 3, 30]), "letzter sonntag im märz 2014");
    example!(v, check_moment!(c, [2013, 10, 3]), "dritter tag im oktober");
    example!(v, check_moment!(c, [2014, 10, 6], Grain::Week), "erste woche im oktober 2014");
    example!(v, check_moment!(c, [2015, 10, 31]), "letzter tag im oktober 2015");
    example!(v, check_moment!(c, [2014, 9, 22], Grain::Week), "letzte woche im september 2014");
    // nth of
    example!(v, check_moment!(c, [2013, 10, 1]), "erster dienstag im oktober");
    example!(v, check_moment!(c, [2014, 9, 16]), "dritter dienstag im september 2014");
    example!(v, check_moment!(c, [2014, 10, 1]), "erster mittwoch im oktober 2014");
    example!(v, check_moment!(c, [2014, 10, 8]), "zweiter mittwoch im oktober 2014");
    // nth after
    example!(v, check_moment!(c, [2015, 1, 13]), "dritter dienstag nach weihnachten 2014");
    example!(v, check_moment!(c, [2013, 2, 12, 3]), "um 3 in der früh");
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "um 3", "3 uhr", "um drei");
    // TODO Check this example
    //example!(v, check_moment!(c, [2013, 2, 13, 3, 18]), "3:18 fruh", "3:18"); //"3:18 früh", "3:18"
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "um 3 am nachmittag", "um 15", "um 15 uhr", "15 uhr");
    example!(v, check_moment_with_precision!(c, [2013, 2, 12, 15], Precision::Approximate), "zirka 15 uhr", "zirka 3 uhr am nachmittag", "um ungefahr 15 uhr"); // "zirka 15 uhr", "zirka 3 uhr am nachmittag", "um ungefähr 15 uhr"
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "um 3 am nachmittag", "um 15", "um 15 uhr", "15 uhr");
    example!(v, check_moment!(c, [2013, 2, 13, 17]), "pünktlich um 17 uhr morgen");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 15]), "um viertel nach 3", "viertel nach drei Uhr", "3 uhr 15 am nachmittag", "15:15");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 20]), "um 20 nach 3", "15:20 am nachmittag", "15 uhr 20 nachmittags", "zwanzig nach 3", "15:20");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 30]), "um halb 4");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 30]), "halb vier uhr nachmittags", "halb vier am nachmittag", "15:30");
    example!(v, check_moment!(c, [2013, 2, 13, 3, 30]), "3:30");
    example!(v, check_moment!(c, [2013, 2, 12, 11, 45]), "viertel vor 12", "11:45");
    example!(v, check_moment!(c, [2013, 2, 12, 11, 45], Grain::Second), "15 minuten vor 12");
    example!(v, check_moment!(c, [2013, 2, 12, 20]), "8 uhr am abend", "heute abend um 20 Uhr");
    example!(v, check_moment!(c, [2013, 2, 12, 20, 00]), "heute um 20:00");
    example!(v, check_moment!(c, [2013, 9, 20, 19, 30]), "um 19:30 am fr, 20. Sept.");
    example!(v, check_moment!(c, [2013, 2, 16, 9]), "am samstag um 9 Uhr");
    example!(v, check_moment!(c, [2014, 7, 18, 19]), "Fr, 18. Juli 2014 7 uhr abends");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 1]), "in einer sekunde");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 31, 0]), "in einer minute");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 32, 0]), "in 2 minuten");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30, 0]), "in 60 minuten");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 0, 0]), "in einer halben stunde", "in 30 minuten");
    example!(v, check_moment!(c, [2013, 2, 12, 7, 0, 0]), "in 2.5 stunden", "in zwei ein halb stunden");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30]), "in einer stunde");
    example!(v, check_moment!(c, [2013, 2, 12, 6, 30]), "in zwei stunden");
    example!(v, check_moment!(c, [2013, 2, 12, 6, 30]), "in ein paar stunden");
    example!(v, check_moment!(c, [2013, 2, 13, 4, 30]), "in 24 stunden");
    example!(v, check_moment!(c, [2013, 2, 13]), "morgen");
    example!(v, check_moment!(c, [2016, 2]), "in 3 Jahren");
    example!(v, check_moment!(c, [2013, 2, 19, 4]), "in 7 tagen");
    example!(v, check_moment!(c, [2013, 2, 19]), "in 1 woche", "in einer woche");
    example!(v, check_moment_with_precision!(c, [2013, 2, 12, 5, 0, 0], Precision::Approximate), "in zirka einer halben stunde");
    example!(v, check_moment!(c, [2013, 2, 5, 4]), "vor 7 tagen");
    example!(v, check_moment!(c, [2013, 1, 29, 4]), "vor 14 tagen");
    example!(v, check_moment!(c, [2013, 1, 29]), "vor zwei wochen");
    example!(v, check_moment!(c, [2013, 2, 5]), "vor einer woche");
    example!(v, check_moment!(c, [2013, 1, 22]), "vor drei wochen");
    example!(v, check_moment!(c, [2012, 11, 12]), "vor drei monaten");
    example!(v, check_moment!(c, [2011, 2]), "vor zwei jahren");
    example!(v, check_moment!(c, [2013, 2, 19, 4]), "in 7 tagen");
    example!(v, check_moment!(c, [2013, 12]), "ein jahr nach weihnachten");
    example!(v, check_moment_span!(c, [2013, 6, 21], [2013, 9, 24]), "diesen sommer");
    example!(v, check_moment_span!(c, [2012, 12, 21], [2013, 3, 21]), "diesen winter");
    example!(v, check_moment!(c, [2013, 12, 25]), "Weihnachten", "Weihnachtstag");
    example!(v, check_moment!(c, [2013, 12, 31]), "Silvester");
    example!(v, check_moment!(c, [2014, 1, 1]), "Neujahrstag", "Neujahr");
    example!(v, check_moment!(c, [2013, 2, 14]), "Valentinstag");
    example!(v, check_moment!(c, [2013, 5, 12]), "Muttertag");
    //example!(v, check_moment!(c, [2013, 6, 16]), "Vatertag"); // TODO Lunear Calendar
    example!(v, check_moment!(c, [2013, 10, 3]), "Tag der Deutschen Einheit", "3. Oktober");
    example!(v, check_moment!(c, [2013, 10, 31]), "Halloween");
    example!(v, check_moment!(c, [2013, 11, 1]), "Allerheiligen");
    example!(v, check_moment!(c, [2013, 12, 06]), "Nikolaus", "Nikolaustag");
    example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 13, 00]), "heute abend", "am abend");
    example!(v, check_moment_span!(c, [2013, 2, 13, 18], [2013, 2, 14, 00]), "morgen abend");
    example!(v, check_moment!(c, [2013, 2, 13, 12]), "morgen mittag", "morgen zu mittag");
    example!(v, check_moment_span!(c, [2013, 2, 11, 18], [2013, 2, 12, 00]), "gestern abend");
    example!(v, check_moment_span!(c, [2013, 2, 15, 18], [2013, 2, 18, 00]), "dieses wochenende");
    example!(v, check_moment_span!(c, [2013, 2, 18, 3], [2013, 2, 18, 12]), "montag morgens");
    example!(v, check_moment_span!(c, [2013, 2, 15, 3], [2013, 2, 15, 12]), "morgens am 15. februar", "15. februar morgens", "am morgen des 15. februar");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 29, 58], [2013, 2, 12, 4, 30, 00]), "letzte 2 sekunden", "letzten zwei sekunden");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 01], [2013, 2, 12, 4, 30, 04]), "nachste 3 sekunden", "nachsten drei sekunden", "nächste 3 sekunden", "nächsten drei sekunden");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 28], [2013, 2, 12, 4, 30]), "letzte 2 minuten", "letzten zwei minuten");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 31], [2013, 2, 12, 4, 34]), "nachste 3 minuten", "nachsten drei minuten", "nächste 3 minuten", "nächsten drei minuten");
    example!(v, check_moment_span!(c, [2013, 2, 12, 5], [2013, 2, 12, 8]), "nächste 3 stunden", "nächsten drei stunden");
    example!(v, check_moment_span!(c, [2013, 2, 10], [2013, 2, 12]), "letzte 2 tage", "letzten zwei tage", "vergangenen zwei tage");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 16]), "nächsten 3 tagen", "nächsten drei tage", "kommenden drei tagen");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 15]), "nächsten paar tagen", "kommenden paar tagen");
    example!(v, check_moment_span!(c, [2013, 1, 28], [2013, 2, 11], Grain::Week), "letzten 2 wochen", "letzte zwei wochen", "vergangenen 2 wochen");
    example!(v, check_moment_span!(c, [2013, 2, 18], [2013, 3, 11], Grain::Week), "nächsten 3 wochen", "nächste drei wochen", "kommenden drei wochen");
    example!(v, check_moment_span!(c, [2012, 12], [2013, 02]), "letzten 2 monaten", "letzte zwei monate", "vergangenen zwei monaten");
    example!(v, check_moment_span!(c, [2013, 3], [2013, 6]), "nächsten 3 monaten", "nächste drei monate", "kommenden drei monaten");
    example!(v, check_moment_span!(c, [2011], [2013]), "letzten 2 jahren", "letzten zwei jahre", "vergangenen zwei jahren");
    example!(v, check_moment_span!(c, [2014], [2017]), "nächsten 3 jahren", "kommenden drei jahren", "nächste drei jahre");
    example!(v, check_moment_span!(c, [2013, 7, 13], [2013, 7, 16]), "13. - 15. Juli", "13ter bis 15ter Juli", "13 bis 15 Juli", "13 - 15 Juli", "Juli 13 - Juli 15");
    example!(v, check_moment_span!(c, [2013, 8, 8], [2013, 8, 13]), "Aug 8 - Aug 12");
    example!(v, check_moment_span!(c, [2013, 2, 12, 9, 30], [2013, 2, 12, 11, 1]), "9:30 - 11:00");
    example!(v, check_moment_span!(c, [2013, 2, 14, 9, 30], [2013, 2, 14, 11, 1]),"am Donnerstag von 9:30 - 11:00", "am Donnerstag zwischen 9:30 und 11:00", "Donnerstag 9:30 - 11:00", "am Donnerstag nach 9:30 aber vor 11:00", "Donnerstag von 9:30 bis 11:00");
    example!(v, check_moment_span!(c, [2013, 2, 14, 9], [2013, 2, 14, 12]), "Donnerstag Vormittag von 9 bis 11");
    example!(v, check_moment_span!(c, [2013, 2, 12, 11, 30], [2013, 2, 12, 13, 31]), "11:30-13:30", "11:30-13:30", "11:30-13:30", "11:30-13:30", "11:30-13:30", "11:30-13:30", "11:30-13:30");
    example!(v, check_moment!(c, [2013, 9, 21, 1, 30]), "1:30 am Sa, 21. Sept");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 26]), "binnen 2 wochen", "innerhalb von 2 wochen");
    example!(v, check_moment_with_direction!(c, [2013, 2, 12, 14], Direction::Before), "bis 2 Uhr nachmittag");
    example!(v, check_moment_with_direction!(c, [2013, 2, 13, 0], Direction::Before), "bis zum ende des tages");
    example!(v, check_moment_with_direction!(c, [2013, 3], Direction::Before), "bis zum ende des monats");
    example!(v, check_moment!(c, [2013, 2, 12, 14]), "heute um 14 Uhr", "um 2");
    example!(v, check_moment!(c, [2013, 2, 13, 15]), "morgen um 15 Uhr");
    example!(v, check_moment_with_direction!(c, [2013, 2, 12, 14], Direction::After), "nach 14 Uhr", "nach 14h", "nach 2");
    example!(v, check_moment_with_direction!(c, [2013, 2, 12, 11], Direction::Before), "bis 11 uhr", "bis 11h vormittags", "bis 11 am vormittag");
    example!(v, check_moment_span!(c, [2013, 2, 12, 13], [2013, 2, 12, 19]), "am nachmittag");
    example!(v, check_moment!(c, [2013, 2, 12, 13, 30]), "um 13:30 am nachmittag", "nachmittags um 1 uhr 30", "13:30");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 45, 0]), "in 15 minuten");
    example!(v, check_moment_span!(c, [2013, 2, 12, 13], [2013, 2, 12, 16]), "nach dem mittagessen");
    example!(v, check_moment!(c, [2013, 2, 12, 10, 30]), "10:30");
    example!(v, check_moment_span!(c, [2013, 2, 12, 3], [2013, 2, 12, 12]), "in der früh", "am morgen");
    example!(v, check_moment!(c, [2013, 2, 18]), "nächsten montag", "kommenden montag");
    example!(v, check_moment!(c, [2013, 12, 10]), "10.12.");
    example!(v, check_moment_span!(c, [2013, 2, 12, 18, 30], [2013, 2, 12, 19, 1]), "18:30h - 19:00h");

    // Additional examples
    example!(v, check_moment!(c, [2013, 2, 12, 6, 0, 0]), "in anderthalb stunde");
    example!(v, check_moment!(c, [2013, 2, 12, 6, 0, 0]), "in eineinhalb std", "in eineinhalb Std.", "in der nächsten eineinhalb Stunde");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30]), "in der nächsten Stunde");
    example!(v, check_moment_with_precision!(c, [2013, 3, 5], Precision::Approximate), "in fast drei Wochen");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 45, 0]), "in einer Stunde und eine viertelstunde");
    example!(v, check_moment!(c, [2013, 2, 12, 6, 0, 0]), "in einer Stunde und dreissig minuten");
    example!(v, check_moment!(c, [2013, 2, 12, 7, 30]), "In drei Stunden ab sofort");
    example!(v, check_moment!(c, [2013, 2]), "in diesem Monat", "diesem Monat");
    example!(v, check_moment!(c, [2013]), "in diesem Jahr");
    example!(v, check_moment!(c, [2013, 2, 4], Grain::Week), "vorige woche", "vorherige Woche");
    example!(v, check_moment!(c, [2013, 5, 3]), "den dritten tag in mai");
    example!(v, check_moment!(c, [2014, 1, 20], Grain::Week), "die vierte Woche nach Weihnachten");
    //example!(v, check_moment!(c, [2013, 6, ]), "männertag"); // TODO Lunear Calendar
    example!(v, check_moment!(c, [2017, 5, 12, 10, 32]), "Freitag, der Zwölfte Mai um 10 Uhr 32 vormittags");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4], [2013, 2, 12, 9]), "am frühen vormittag", "bei tagesanbruch", "beim morgengrauen", "im morgengrauen", "in der morgenfrühe", "frühmorgens", "am frühen morgen");
    example!(v, check_moment_span!(c, [2013, 2, 12, 11], [2013, 2, 12, 13]), "kurz vor mittag", "am späten vormittag");
    example!(v, check_moment_span!(c, [2013, 2, 12, 17], [2013, 2, 12, 19]), "am späten nachmittag","in den späten nachmittagsstunden","zu später nachmittagsstunde","spätnachmittags","spätnachmittag");
    example!(v, check_moment!(c, [2013, 7, 15], Grain::Week), "die dritte juliwoche");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4], [2013, 2, 12, 11, 31]), "ab dem frühen vormittag bis nach halb zwölf");
    example!(v, check_moment!(c, [2013, 2, 12, 11, 15]), "um viertel mittag");
}

pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(0), "0", "null");
    example!(v, check_integer(1), "1", "eins");
    example!(v, check_integer(3), "3", "drei");
    example!(v, check_integer(30), "30", "dreissig");
    example!(v, check_integer(33), "33", "drei und dreissig", "dreiunddreissig", "0033");
    example!(v, check_integer(14), "14", "vierzehn");
    example!(v, check_integer(16), "16", "sechzehn");
    example!(v, check_integer(17), "17", "siebzehn");
    example!(v, check_integer(18), "18", "achtzehn");
    example!(v, check_integer(200), "200", "zwei hundert");
    example!(v, check_integer(102), "102", "hundert zwei");
    example!(v, check_float(1.1), "1,1", "1 komma 1", "1,10", "01,10");
    example!(v, check_float(0.77), "0,77", ",77");
    example!(v, check_integer(100000), "100.000", "100000", "100K", "100k");
    example!(v, check_integer(3000000), "3M", "3000K", "3000000", "3.000.000");
    example!(v, check_integer(1200000), "1.200.000", "1200000", "1,2M", "1200K", ",0012G");
    example!(v, check_integer(-1200000), "- 1.200.000", "-1200000", "minus 1.200.000", "negativ 1200000", "-1,2M", "-1200K", "-,0012G");
    example!(v, check_integer(5000), "5 tausend", "fünf tausend");
    example!(v, check_integer(200000), "zwei hundert tausend");
    example!(v, check_integer(21311), "ein und zwanzig tausend drei hundert elf");
    example!(v, check_integer(721012), "sieben hundert einundzwanzig tausend zwölf");
    example!(v, check_integer(31256721), "ein und dreissig millionen zwei hundert sechs und fünfzig tausend sieben hundert ein und zwanzig");
    example!(v, check_ordinal(4), "vierter", "4ter");
    example!(v, check_float(1416.15), "1416,15");
    example!(v, check_float(1416.15), "1.416,15");
    example!(v, check_float(1000000.0), "1.000.000,00");
    example!(v, check_ordinal(44), "der vierundvierzigste");
    example!(v, check_integer(455628), "vierhundertfünfundfünfzigtausendsechshundertachtundzwanzig");
}
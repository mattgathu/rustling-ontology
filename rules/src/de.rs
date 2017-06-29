use rustling::*;
use values::dimension::*;
use values::dimension::Precision::*;
use values::helpers;
use moment::{Grain, PeriodComp, Weekday};

pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("second (unit-of-duration)",
        b.reg(r#"sek(?:unden?|\.?)|s(?:ec)?\.?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );
    b.rule_1("minute (unit-of-duration)",
        b.reg(r#"min(?:uten?|\.?)"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Minute))
    );
    b.rule_1("hour (unit-of-duration)",
        b.reg(r#"st(?:unden?|dn?\.?)"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Hour))
    );
    b.rule_1("day (unit-of-duration)",
        b.reg(r#"tage?n?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Day))
    );
    b.rule_1("week (unit-of-duration)",
        b.reg(r#"wochen?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Week))
    );
    b.rule_1("month (unit-of-duration)",
        b.reg(r#"monate?n?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Month))
    );
    b.rule_1("year (unit-of-duration)",
        b.reg(r#"jahre?n?s?"#)?,
        |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );
    b.rule_2("few unit of duration",
        b.reg(r#"wenigen"#)?,
        unit_of_duration_check!(),
        |_, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, 3).into()))
    );
    b.rule_1("1/4 hour",
        b.reg(r#"(?:1/4\s?|(?:eine?r? )viertel)stunde"#)?,
        |_| Ok(DurationValue::new(PeriodComp::minutes(15).into()))
    );
    b.rule_1("half an hour",
        b.reg(r#"(?:1/2\s?|(?:eine?r? )halbe?n? )stunde"#)?,
        |_| Ok(DurationValue::new(PeriodComp::minutes(30).into()))
    );
    b.rule_1("3/4 hour",
        b.reg(r#"(?:3/4\s?|(?:eine?r? )dreiviertel)stunde"#)?,
        |_| Ok(DurationValue::new(PeriodComp::minutes(45).into()))
    );
    b.rule_2("a <duration>",
        b.reg(r#"(?:in )?eine?(?:r|n)?"#)?,
        duration_check!(),
        |_, duration| duration.value().in_present() 
    );
    b.rule_2("<integer> <unit-of-duration>",
        integer_check!(0),
        unit_of_duration_check!(),
        |integer, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_2("number.number hours",
        b.reg(r#"(\d+)\.(\d+)"#)?,
        unit_of_duration_check!(|uod: &UnitOfDurationValue| uod.grain == Grain::Hour),
        |text_match, _| Ok(DurationValue::new(
                    PeriodComp::new(
                        Grain::Minute, 
                        helpers::decimal_hour_in_minute(text_match.group(1), text_match.group(2))?
                    ).into()
                ))
    );
    b.rule_2("1..12 and an half hour",
        b.reg(r#"(ein|zwei|drei|vier|f[üu]nf|sechs|sieben|acht|neun|zehn|elf|zw[öo]lf)einhalb"#)?,
        unit_of_duration_check!(|uod: &UnitOfDurationValue| uod.grain == Grain::Hour),
        |text_match, _| {
            let value = match text_match.group(1).as_ref() {
                "ein" => 1,
                "zwei" => 2,
                "drei" => 3,
                "vier" => 4,
                "funf" => 5,
                "fünf" => 5,
                "sechs" => 6,
                "sieben" => 7,
                "acht" => 8,
                "neun" => 9,
                "zehn" => 10,
                "elf" => 11,
                "zwolf" => 12,
                "zwölf" => 12,
                _ => panic!("No match found for: {:?}", text_match),
            };
            Ok(DurationValue::new(PeriodComp::minutes(value * 60 + 30).into()))
        }
    );
    b.rule_2("half an hour",
        b.reg(r#"anderthalb"#)?,
        unit_of_duration_check!(|uod: &UnitOfDurationValue| uod.grain == Grain::Hour),
        |_, _| Ok(DurationValue::new(PeriodComp::minutes(90).into()))
    );
    b.rule_3("<integer> and an half hours",
        integer_check!(0),
        b.reg(r#"ein ?halb"#)?,
        unit_of_duration_check!(|uod: &UnitOfDurationValue| uod.grain == Grain::Hour),
        |integer, _, _| Ok(DurationValue::new(PeriodComp::minutes(integer.value().value * 60 + 30).into()))
    );
    b.rule_2("a <unit-of-duration>",
        b.reg(r#"eine?(?:r|n)?"#)?,
        unit_of_duration_check!(),
        |_, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, 1).into()))
    );
    b.rule_2("in next <unit-of-duration>",
        b.reg(r#"in de(?:n|r|m) (?:n[äa]chste(?:n|r|m)|kommende(?:r|n|m))"#)?,
        unit_of_duration_check!(),
        |_, uod| DurationValue::new(PeriodComp::new(uod.value().grain, 1).into())
                    .in_present()
    );
    b.rule_2("in <duration>",
        b.reg(r#"in(?:\s(?:de(?:n|r|m)\s)?(?:n[äa]chste(?:n|r|m)|kommende(?:r|n|m)))?"#)?,
        duration_check!(),
        |_, duration| duration.value().in_present()
    );
    b.rule_2("after <duration>",
        b.reg(r#"nach"#)?,
        duration_check!(),
        |_, duration| duration.value().in_present()
    );
    b.rule_2("<duration> from now",
        duration_check!(),
        b.reg(r#"ab (?:heute|jetzt|sofort)"#)?,
        |duration, _| duration.value().in_present()
    );
    b.rule_3("in <duration> from now",
        b.reg(r#"in"#)?,
        duration_check!(),
        b.reg(r#"ab (?:heute|jetzt|sofort)"#)?,
        |_, duration, _| duration.value().in_present()
    );
    b.rule_2("<duration> ago",
        b.reg(r#"vor"#)?,
        duration_check!(),
        |_, duration| duration.value().ago()
    );
    b.rule_3("<duration> after <time>",
        duration_check!(),
        b.reg(r#"nach"#)?,
        time_check!(),
        |duration, _, time| duration.value().after(time.value())
    );
    b.rule_3("<duration> before <time>",
        duration_check!(),
        b.reg(r#"vor"#)?,
        time_check!(),
        |duration, _, time| duration.value().before(time.value())
    );
    b.rule_2("about <duration>",
        b.reg(r#"ungef[äa]hr|zirka|circa|ca.|etwa|fast"#)?,
        duration_check!(),
        |_, duration| Ok(duration.value().clone().precision(Approximate))
    );
    b.rule_2("exactly <duration>",
        b.reg(r#"genau|exakt"#)?,
        duration_check!(),
        |_, duration| Ok(duration.value().clone().precision(Exact))
    );
    b.rule_3("<duration> and <duration>",
        duration_check!(),
        b.reg(r#"und"#)?,
        duration_check!(),
        |a, _, b| Ok(a.value() + b.value())
    );
    b.rule_2("<duration> <duration>",
        duration_check!(),
        duration_check!(),
        |a, b| Ok(a.value() + b.value())
    );
    Ok(())
}

pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("second (cycle)",
        b.reg(r#"sekunden?"#)?,
        |_| CycleValue::new(Grain::Second)
    );
    b.rule_1("minute (cycle)",
        b.reg(r#"minuten?"#)?,
        |_| CycleValue::new(Grain::Minute)
    );
    b.rule_1("hour (cycle)",
        b.reg(r#"stunden?"#)?,
        |_| CycleValue::new(Grain::Hour)
    );
    b.rule_1("day (cycle)",
        b.reg(r#"tage?n?s?"#)?,
        |_| CycleValue::new(Grain::Day)
    );
    b.rule_1("week (cycle)",
        b.reg(r#"wochen?"#)?,
        |_| CycleValue::new(Grain::Week)
    );
    b.rule_1("month (cycle)",
        b.reg(r#"monate?n?s?"#)?,
        |_| CycleValue::new(Grain::Month)
    );
    b.rule_1("quarter (cycle)",
        b.reg(r#"quartale?"#)?,
        |_| CycleValue::new(Grain::Quarter)
    );
    b.rule_1("year (cycle)",
        b.reg(r#"jahre?n?s?"#)?,
        |_| CycleValue::new(Grain::Year)
    );
    b.rule_2("this <cycle>",
        b.reg(r#"(?:in )?diese(?:r|n|s|m)?|kommende(?:r|n|s|m)?"#)?,
        cycle_check!(),
        |_, cycle| helpers::cycle_nth(cycle.value().grain, 0)
    );
    b.rule_2("last <cycle>",
        b.reg(r#"letzte(?:r|n|s)?|vergangene(?:r|n|s)?|vor(?:her)?ige(?:r|n|s)?"#)?,
        cycle_check!(),
        |_, cycle| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_2("next <cycle>",
        b.reg(r#"n[äa]chste(?:r|n|s)?|kommende(?:r|n|s)?"#)?,
        cycle_check!(),
        |_, cycle| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_4("the <cycle> after <time>",
        b.reg(r#"de(?:r|n|m|s)|die|das"#)?,
        cycle_check!(),
        b.reg(r#"nach"#)?,
        time_check!(),
        |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, 1, time.value())
    );
    b.rule_4("the <cycle> before <time>",
        b.reg(r#"de(?:r|n|m|s)|die|das"#)?,
        cycle_check!(),
        b.reg(r#"vor"#)?,
        time_check!(),
        |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, -1, time.value())
    );
    b.rule_3("last n <cycle>",
        b.reg(r#"letzten?|vergangenen?"#)?,
        integer_check!(1, 9999),
        cycle_check!(),
        |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("next n <cycle>",
        b.reg(r#"n[äa]chsten?|kommenden?"#)?,
        integer_check!(1, 9999),
        cycle_check!(),
        |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_4("<ordinal> <cycle> of/nach <time>",
        ordinal_check!(),
        cycle_check!(),
        b.reg(r#"im|in|von|nach"#)?,
        time_check!(),
        |ordinal, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, time.value())
    );
    b.rule_2("<ordinal> quarter",
        ordinal_check!(),
        cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
        |ordinal, _| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, &helpers::cycle_nth(Grain::Year, 0)?)
    );
    b.rule_3("<ordinal> quarter <year>",
        ordinal_check!(),
        cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
        time_check!(),
        |ordinal, _, time| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, time.value())
    );
    Ok(())
}

pub fn rules_time(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
        time_check!(|time: &TimeValue| !time.latent),
        time_check!(|time: &TimeValue| !time.latent),
        |a, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by 'of', 'from', 's",
        time_check!(|time: &TimeValue| !time.latent),
        b.reg(r#"von|de(?:r|s|n|m)|im"#)?,
        time_check!(|time: &TimeValue| !time.latent),
        |a, _, b| a.value().intersect(b.value())
    );

    b.rule_3("intersect by ','",
        time_check!(|time: &TimeValue| !time.latent),
        b.reg(r#","#)?,
        time_check!(|time: &TimeValue| !time.latent),
        |a, _, b| a.value().intersect(b.value())
    );
    b.rule_2("on <date>",
        b.reg(r#"am"#)?,
        time_check!(),
        |_, time| Ok(time.value().clone())
    );
    b.rule_2("on a named-day",
        b.reg(r#"an einem|an dem"#)?,
        time_check!(form!(Form::DayOfWeek{..})),
        |_, time| Ok(time.value().clone())
    );
    b.rule_1("named-day",
        b.reg(r#"montags?|mo\.?"#)?,
        |_| helpers::day_of_week(Weekday::Mon)
    );
    b.rule_1("named-day",
        b.reg(r#"die?nstags?|di\.?"#)?,
        |_| helpers::day_of_week(Weekday::Tue)
    );
    b.rule_1("named-day",
        b.reg(r#"mittwochs?|mi\.?"#)?,
        |_| helpers::day_of_week(Weekday::Wed)
    );
    b.rule_1("named-day",
        b.reg(r#"donn?erstags?|do\.?"#)?,
        |_| helpers::day_of_week(Weekday::Thu)
    );
    b.rule_1("named-day",
        b.reg(r#"freitags?|fr\.?"#)?,
        |_| helpers::day_of_week(Weekday::Fri)
    );
    b.rule_1("named-day",
        b.reg(r#"samstags?|sa\.?"#)?,
        |_| helpers::day_of_week(Weekday::Sat)
    );
    b.rule_1("named-day",
        b.reg(r#"sonntags?|so\.?"#)?,
        |_| helpers::day_of_week(Weekday::Sun)
    );
    b.rule_1("named-month",
        b.reg(r#"januars?|j[äa]nners?|j[äa]n\.?"#)?,
        |_| helpers::month(1)
    );
    b.rule_1("named-month",
        b.reg(r#"februars?|feb\.?"#)?,
        |_| helpers::month(2)
    );
    b.rule_1("named-month",
        b.reg(r#"m[äa]rz(?:es)?|m[äa]r\.?"#)?,
        |_| helpers::month(3)
    );
    b.rule_1("named-month",
        b.reg(r#"aprils?|apr\.?"#)?,
        |_| helpers::month(4)
    );
    b.rule_1("named-month",
        b.reg(r#"maie?s?"#)?,
        |_| helpers::month(5)
    );
    b.rule_1("named-month",
        b.reg(r#"junis?|jun\.?"#)?,
        |_| helpers::month(6)
    );
    b.rule_1("named-month",
        b.reg(r#"julis?|jul\.?"#)?,
        |_| helpers::month(7)
    );
    b.rule_1("named-month",
        b.reg(r#"auguste?s?|aug\.?"#)?,
        |_| helpers::month(8)
    );
    b.rule_1("named-month",
        b.reg(r#"septembers?|sept?\.?"#)?,
        |_| helpers::month(9)
    );
    b.rule_1("named-month",
        b.reg(r#"oktobers?|okt\.?"#)?,
        |_| helpers::month(10)
    );
    b.rule_1("named-month",
        b.reg(r#"novembers?|nov\.?"#)?,
        |_| helpers::month(11)
    );
    b.rule_1("named-month",
        b.reg(r#"dezembers?|dez\.?"#)?,
        |_| helpers::month(12)
    );
    b.rule_1("christmas",
        b.reg(r#"weih?nacht(?:en|s(?:feier)?tag)?"#)?,
        |_| helpers::month_day(12, 25)
    );
    b.rule_1("christmas eve",
        b.reg(r#"heilig(er)? abend"#)?,
        |_| helpers::month_day(12, 24)
    );
    b.rule_1("new year's eve",
        b.reg(r#"silvester|neujahrsabend"#)?,
        |_| helpers::month_day(12, 31)
    );
    b.rule_1("new year's day",
        b.reg(r#"neujahr(?:s?tag)?"#)?,
        |_| helpers::month_day(1, 1)
    );
    b.rule_1("valentine's day",
        b.reg(r#"valentin'?stag"#)?,
        |_| helpers::month_day(2, 14)
    );
    b.rule_1("Tag der Deutschen Einheit",
        b.reg(r#"tag (?:der)? deutsc?hen? einheit"#)?,
        |_| helpers::month_day(10, 3)
    );
    b.rule_1("Österreichischer Nationalfeiertag",
        b.reg(r#"([öo]sterreichischer?)? nationalfeiertag|national feiertag"#)?,
        |_| helpers::month_day(10, 26)
    );
    b.rule_1("Schweizer Bundesfeiertag",
        b.reg(r#"schweiz(?:er)? (?:bundes)?feiertag|bundes feiertag"#)?,
        |_| helpers::month_day(8, 1)
    );
    
    // TODO needs the lunar calendar feature
    // b.rule_1("Ascension celebration",
    //     b.reg(r#"himmelfahrt"#)?,
    //     |_| 
    // );

    // TODO in Germany it is the same day as the ascension celebration
    // b.rule_1("Father's Day",  // third Sunday of June
    //     b.reg(r#"vatt?er(?: ?tag)?|(?:herren|m[äa]nner)tag"#)?,
    //     |_| helpers::day_of_week(Weekday::Sun)?
    //             .intersect(&helpers::month(6)?)?
    //             .intersect(&helpers::cycle_nth_after(Grain::Week, 2, &helpers::month_day(6, 1)?)?)
    // );
    b.rule_1("Mother's Day",
        b.reg(r#"mutt?ertag|mutt?er (?:tag)?"#)?,
        |_| helpers::day_of_week(Weekday::Sun)?
                .intersect(&helpers::month(5)?)?
                .intersect(&helpers::cycle_nth_after(Grain::Week, 1, &helpers::month_day(5, 1)?)?)
    );
    b.rule_1("halloween day",
        b.reg(r#"hall?owe?en?"#)?,
        |_| helpers::month_day(10, 31)
    );
    b.rule_1("Allerheiligen",
        b.reg(r#"allerheiligen?|aller heiligen?"#)?,
        |_| helpers::month_day(11, 1)
    );
    b.rule_1("Nikolaus",
        b.reg(r#"nikolaus(?: ?tag|abend)?|nikolo"#)?,
        |_| helpers::month_day(12, 6)
    );
    // b.rule_2("absorption of , after named day",
    //     time_check!(form!(Form::DayOfWeek{..})),
    //     b.reg(r#","#)?,
    //     |time, _| Ok(time.value().clone())
    // );
    b.rule_1("now",
        b.reg(r#"(?:genau ?)?jetzt|diesen moment|in diesem moment|gerade (?:eben|jetzt)"#)?,
        |_| helpers::cycle_nth(Grain::Second, 0)
    );
    b.rule_1("today",
        b.reg(r#"heute?|um diese zeit|zu dieser zeit|um diesen zeitpunkt|zu diesem zeitpunkt|derzeitig|momentan|zurzeit"#)?,
        |_| helpers::cycle_nth(Grain::Day, 0)
    );
    b.rule_1("tomorrow",
        b.reg(r#"morgen"#)?,
        |_| helpers::cycle_nth(Grain::Day, 1)
    );
    b.rule_1("after tomorrow",
        b.reg(r#"[üu]bermorgen"#)?,
        |_| helpers::cycle_nth(Grain::Day, 2)
    );
    b.rule_1("after after tomorrow",
        b.reg(r#"[üu]ber[üu]bermorgen"#)?,
        |_| helpers::cycle_nth(Grain::Day, 3)
    );
    b.rule_1("yesterday",
        b.reg(r#"gestern"#)?,
        |_| helpers::cycle_nth(Grain::Day, -1)
    );
    b.rule_1("before yesterday",
        b.reg(r#"vorgestern"#)?,
        |_| helpers::cycle_nth(Grain::Day, -2)
    );
    b.rule_1("before before yesterday",
        b.reg(r#"vorvorgestern"#)?,
        |_| helpers::cycle_nth(Grain::Day, -3)
    );
    b.rule_1("EOM|End of month",
        b.reg(r#"(?:(?:das|am) )?ende (?:des|vom) monate?s?|monatsende"#)?,
        |_| helpers::cycle_nth(Grain::Month, 1)
    );
    b.rule_1("EOY|End of year",
        b.reg(r#"(?:das )?(?:eoy|jahr(?:es)?(?:ende|schluss)|ende (?:des|vom) jahr(?:e?s)?)"#)?,
        |_| helpers::cycle_nth(Grain::Year, 1)
    );
    b.rule_2("this|next <day-of-week>",
        b.reg(r#"diese(?:n|r)|kommenden|n[äa]chsten"#)?,
        time_check!(form!(Form::DayOfWeek{..})),
        |_, time| time.value().the_nth_not_immediate(0)
    );
    b.rule_2("this <time>",
        b.reg(r#"diese(?:n|r|s|m)?|(?:im )?laufende(?:n|r|s)"#)?,
        time_check!(),
        |_, time| time.value().the_nth(0)
    );
    b.rule_2("next <time>",
        b.reg(r#"(?:n[äa]chst|kommend)e(?:n|s|r|m)?"#)?,
        time_check!(),
        |_, time| time.value().the_nth_not_immediate(0)
    );
    b.rule_2("last <time>",
        b.reg(r#"(?:letzt|vor(?:her)?ig)e(?:n|s|m|r)?"#)?,
        time_check!(),
        |_, time| time.value().the_nth(-1)
    );
    b.rule_2("after next <time>",
        b.reg(r#"[üu]ber ?n[äa]chste(?:r|s|n|m)?"#)?,
        time_check!(),
        |_, time| time.value().the_nth_not_immediate(1)
    );
    b.rule_2("<time> after next",
        time_check!(),
        b.reg(r#"nach de(?:m|r|n) n[äa]chsten"#)?,
        |time, _| time.value().the_nth_not_immediate(1)
    );
    b.rule_2("<time> before last",
        b.reg(r#"vor ?letzte(?:n|s|m|r)?"#)?,
        time_check!(),
        |_, time| time.value().the_nth(-2)
    );
    b.rule_4("last <day-of-week> of <time>",
        b.reg(r#"letzte(?:r|n|s)?"#)?,
        time_check!(form!(Form::DayOfWeek{..})),
        b.reg(r#"um|im|in der"#)?,
        time_check!(),
        |_, dow, _, time| dow.value().last_of(time.value())
    );
    b.rule_4("last <cycle> of <time>",
        b.reg(r#"letzte(?:r|n|s)?"#)?,
        cycle_check!(),
        b.reg(r#"um|im|in der"#)?,
        time_check!(),
        |_, cycle, _, time| cycle.value().last_of(time.value())
    );
    b.rule_5("the last <cycle> of <time>",
        b.reg(r#"de(?:r|s|n|m)"#)?,
        b.reg(r#"letzte(?:r|n|s)?"#)?,
        cycle_check!(),
        b.reg(r#"um|im|in der"#)?,
        time_check!(),
        |_, _, cycle, _, time| cycle.value().last_of(time.value())
    );
    b.rule_4("nth <time> of <time>",
        ordinal_check!(),
        time_check!(),
        b.reg(r#"im|in de(?:r|n)|de(?:s|n)"#)?,
        time_check!(),
        |ordinal, a, _, b| b.value()
                .intersect(a.value())?
                .the_nth(ordinal.value().value - 1)
    );
    b.rule_4("nth <time> after <time>",
        ordinal_check!(),
        time_check!(),
        b.reg(r#"nach"#)?,
        time_check!(),
        |ordinal, a, _, b| a.value().the_nth_after(ordinal.value().value - 1, b.value())
    );
    b.rule_2("in <month>",
        b.reg(r#"im"#)?,
        time_check!(form!(Form::Month(_))),
        |_, month| Ok(month.value().clone())
    );
    b.rule_2("in <year>",
        b.reg(r#"im(?: jahre?n?)|in"#)?,
        time_check!(form!(Form::Year(_))),
        |_, year| Ok(year.value().clone())
    );
    b.rule_1("year",
        integer_check!(1900, 2100),
        |integer| {
            helpers::year(integer.value().value as i32)
        }
    );  
    b.rule_1("year (latent)",
        integer_check!(-1000, 1899),
        |integer| {
            Ok(helpers::year(integer.value().value as i32)?.latent())
        }
    );
    b.rule_1("year (latent)",
        integer_check!(2101, 2200),
        |integer| {
            Ok(helpers::year(integer.value().value as i32)?.latent())
        }
    );
    b.rule_2("the <day-of-month> (ordinal)",
        b.reg(r#"de(?:r|n|m|s)"#)?,
        ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
        |_, ordinal| helpers::day_of_month(ordinal.value().value as u32)
    );

    b.rule_1("<day-of-month> (ordinal)",
        ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
        |ordinal| Ok(helpers::day_of_month(ordinal.value().value as u32)?.latent()) 
    );
    // TODO This rule seems to never happen (should be with an ordinal token)
    b.rule_2("the <day-of-month> (non ordinal)",
        b.reg(r#"de(?:r|n|m|s)"#)?,
        integer_check!(1, 31),
        |_, integer| Ok(helpers::day_of_month(integer.value().value as u32)?.latent())
    );
    b.rule_2("<named-month> <day-of-month> (ordinal)",
        time_check!(form!(Form::Month(_))),
        ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
        |time, ordinal| time.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)
    );
    b.rule_2("<named-month> <day-of-month> (non ordinal)",
        time_check!(form!(Form::Month(_))),
        integer_check!(1, 31),
        |time, integer| time.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_3("<day-of-month> (non ordinal) of <named-month>",
        integer_check!(1, 31),
        b.reg(r#"vom|von|im"#)?,
        time_check!(form!(Form::Month(_))),
        |integer, _, month| month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_2("<day-of-month> (non ordinal) <named-month>",
        integer_check!(1, 31),
        time_check!(form!(Form::Month(_))),
        |integer, month| month.value()
            .intersect(&helpers::day_of_month(integer.value().value as u32)?)
    );
    b.rule_2("<day-of-month>(ordinal) <named-month>",
        ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
        time_check!(form!(Form::Month(_))),
        |ordinal, month| month.value()
            .intersect(&helpers::day_of_month(ordinal.value().value as u32)?)
    );

    b.rule_3("<day-of-month>(ordinal) <named-month> year",
        ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
        time_check!(form!(Form::Month(_))),
        b.reg(r#"(\d{2,4})"#)?,
        |ordinal, month, text_match| month.value()
            .intersect(&helpers::day_of_month(ordinal.value().value as u32)?)?
            .intersect(&helpers::year(text_match.group(1).parse()?)?)
    );
    b.rule_2("the ides of <named-month>",
        b.reg(r#"die iden (des?)"#)?,
        time_check!(form!(Form::Month(_))),
        |_, month| {
            let day_of_month = match month.value().form_month()? {
                3 | 5 | 7 | 10 => 15,
                _ => 13,
            };
            month.value().intersect(&helpers::day_of_month(day_of_month)?)
        }
    );
    b.rule_1("time-of-day (latent)",
        integer_check!(0, 23),
        |integer| Ok(helpers::hour(integer.value().value as u32, integer.value().value < 12)?.latent())
    );
    b.rule_2("<time-of-day> o'clock",
        time_check!(form!(Form::TimeOfDay(_))),
        b.reg(r#"uhr|h|u"#)?,
        |time, _| Ok(time.value().clone().not_latent())
    );
    b.rule_2("at <time-of-day>",
        b.reg(r#"um|@"#)?,
        time_check!(form!(Form::TimeOfDay(_))),
        |_, time| Ok(time.value().clone().not_latent())
    );
    b.rule_1("hh:mm",
        b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.]([0-5]\d)(?:(?i)uhr|h)?"#)?,
        |text_match| Ok(helpers::hour_minute(
                text_match.group(1).parse()?,
                text_match.group(2).parse()?, 
                false)?
            .form(Form::TimeOfDay(None)))
    );
    b.rule_2("<time-of-day> am|pm",
        time_check!(form!(Form::TimeOfDay(_))),
        b.reg(r#"([ap])\.?m\.?"#)?,
        |a, text_match| {
            let day_period = if text_match.group(1) == "a" {
                helpers::hour(0, false)?.span_to(&helpers::hour(12, false)?, false)?
            } else {
                helpers::hour(12, false)?.span_to(&helpers::hour(0, false)?, false)?
            };
            Ok(a.value().intersect(&day_period)?.form(Form::TimeOfDay(None)))
        }
    );
    b.rule_1("noon",
        b.reg(r#"mittags?|zwolf (?:uhr)?"#)?,
        |_| helpers::hour(12, false)
    );
    b.rule_1("midnight|end of day",
        b.reg(r#"mitternacht|tagesende|ende (?:des)? tag(?:es)?"#)?,
        |_| helpers::hour(0, false)
    );
    b.rule_1("quarter (relative minutes)",
        b.reg(r#"vie?rtel"#)?,
        |_| Ok(RelativeMinuteValue(15))
    );
    b.rule_1("half (relative minutes)",
        b.reg(r#"halbe?"#)?,
        |_| Ok(RelativeMinuteValue(30))
    );
    b.rule_1("number (as relative minutes)",
        integer_check!(1, 59),
        |integer| Ok(RelativeMinuteValue(integer.value().value as i32))
    );
    b.rule_2("<hour-of-day> <integer> (as relative minutes)",
        time_check!(form!(Form::TimeOfDay(Some(_)))),
        relative_minute_check!(),
        |time, relative_minute| helpers::hour_relative_minute(
                                        time.value().form_time_of_day()?.full_hour, 
                                        relative_minute.value().0, 
                                        true)
    );
    b.rule_3( "relative minutes to|till|before <integer> (hour-of-day)",
        relative_minute_check!(),
        b.reg(r#"vor"#)?,
        time_check!(form!(Form::TimeOfDay(Some(_)))),
        |relative_minute, _, time| helpers::hour_relative_minute(
                                        time.value().form_time_of_day()?.full_hour, 
                                        -1 * relative_minute.value().0, 
                                        true)
    );
    b.rule_3("relative minutes after|past <integer> (hour-of-day)",
        relative_minute_check!(),
        b.reg(r#"nach"#)?,
        time_check!(form!(Form::TimeOfDay(Some(_)))),
        |relative_minute, _, time| helpers::hour_relative_minute(
                                        time.value().form_time_of_day()?.full_hour, 
                                        relative_minute.value().0, 
                                        true)
    );
    b.rule_2("viertel <integer> (german style hour-of-day)",
        b.reg(r#"vie?rtel"#)?,
        time_check!(form!(Form::TimeOfDay(Some(_)))),
        |_, time| helpers::hour_relative_minute(
                                        time.value().form_time_of_day()?.full_hour, 
                                        15, 
                                        true)
    );
    b.rule_2("half <integer> (german style hour-of-day)",
        b.reg(r#"halbe?"#)?,
        time_check!(form!(Form::TimeOfDay(Some(_)))),
        |_, time| helpers::hour_relative_minute(
                                        time.value().form_time_of_day()?.full_hour, 
                                        -30, 
                                        true)
    );
    b.rule_2("dreiviertel <integer> (german style hour-of-day)",
        b.reg(r#"dreivie?rtel"#)?,
        time_check!(form!(Form::TimeOfDay(Some(_)))),
        |_, time| helpers::hour_relative_minute(
                                        time.value().form_time_of_day()?.full_hour, 
                                        -15, 
                                        true)
    );
    b.rule_1("dd/mm/yyyy",
        b.reg(r#"([012]?[1-9]|10|20|30|31)[\./](0?[1-9]|10|11|12)[\./](\d{2,4})"#)?,
        |text_match| helpers::ymd(
            text_match.group(3).parse()?,
            text_match.group(2).parse()?,
            text_match.group(1).parse()?,
            )
    );
    b.rule_1("dd-mm-yyyy",
        b.reg(r#"([012]?[1-9]|10|20|30|31)-(0?[1-9]|10|11|12)-(\d{2,4})"#)?,
        |text_match| helpers::ymd(
            text_match.group(3).parse()?,
            text_match.group(2).parse()?,
            text_match.group(1).parse()?,
        )
    );
    b.rule_1("mm.dd.",
        b.reg(r#"([012]?[1-9]|10|20|30|31)\.(0?[1-9]|10|11|12)\."#)?,
        |text_match| helpers::month_day(
            text_match.group(2).parse()?,
            text_match.group(1).parse()?)
    );
    b.rule_1("dd/mm",
        b.reg(r#"([012]?[1-9]|10|20|30|31)/(0?[1-9]|10|11|12)"#)?,
        |text_match| helpers::month_day(
            text_match.group(2).parse()?,
            text_match.group(1).parse()?)
    );
    b.rule_1("early morning",
        b.reg(r#"am fr[üu]hen vormittag|bei tagesanbruch|(?:b?e?im|in der) morgen(?:grauen|fr[üu]he)|(?:am )?fr[üu]h(?:en )?morgens?|am morgen fruh"#)?,
        |_| Ok(helpers::hour(4, false)?
                .span_to(&helpers::hour(9, false)?, false)?
                .form(Form::PartOfDay))
    );
    b.rule_1("morning",
        b.reg(r#"morgens|(?:in der )?fr[üu]h|vor ?mittags?|am morgen"#)?,
        |_| Ok(helpers::hour(3, false)?
                .span_to(&helpers::hour(12, false)?, false)?
                .form(Form::PartOfDay))
    );
    b.rule_1("late morning",
        b.reg(r#"(?:kurz|am sp[äa]ten) vor ?mittag"#)?,
        |_| Ok(helpers::hour(11, false)?
                .span_to(&helpers::hour(13, false)?, false)?
                .form(Form::PartOfDay))

    );
    b.rule_1("lunch (latent)",
        b.reg(r#"mittag(?:szeit)?"#)?,
        |_| Ok(helpers::hour(12, false)?
                .span_to(&helpers::hour(14, false)?, false)?
                .latent()
                .form(Form::PartOfDay))
    );
    b.rule_1("lunch",
        b.reg(r#"mittags"#)?,
        |_| Ok(helpers::hour(12, false)?
                .span_to(&helpers::hour(14, false)?, false)?
                .form(Form::PartOfDay))
    );    
    b.rule_1("early afternoon (latent)",
        b.reg(r#"fr[üu]hen nachmittags?(?:stunden)?"#)?,
        |_| Ok(helpers::hour(13, false)?
                .span_to(&helpers::hour(17, false)?, false)?
                .latent()
                .form(Form::PartOfDay))
    );
    b.rule_1("early afternoon",
        b.reg(r#"nach dem mittagessen|kurz nach mittag"#)?,
        |_| Ok(helpers::hour(13, false)?
                .span_to(&helpers::hour(17, false)?, false)?
                .form(Form::PartOfDay))
    );
    b.rule_1("afternoon",
        b.reg(r#"nach ?mittags?"#)?,
        |_| Ok(helpers::hour(13, false)?
                .span_to(&helpers::hour(19, false)?, false)?
                .form(Form::PartOfDay))
    );
    b.rule_1("late afternoon (latent)",
        b.reg(r#"sp[äa]t(?:e[nr] )?nachmittags?(?:stunden?)?"#)?,
        |_| Ok(helpers::hour(17, false)?
                .span_to(&helpers::hour(19, false)?, false)?
                .latent()
                .form(Form::PartOfDay))
    );
    b.rule_1("early evening (latent)",
        b.reg(r#"fr[üu]h am abend"#)?,
        |_| Ok(helpers::hour(18, false)?
                .span_to(&helpers::hour(21, false)?, false)?
                .latent()
                .form(Form::PartOfDay))
    );
    b.rule_1("early evening",
        b.reg(r#"fr[üu]he[nr] abend(?:stunden?)?|abend fr[üu]h"#)?,
        |_| Ok(helpers::hour(18, false)?
                .span_to(&helpers::hour(21, false)?, false)?
                .form(Form::PartOfDay))
    );
    b.rule_1("evening (latent)",
        b.reg(r#"abend"#)?,
        |_| Ok(helpers::hour(18, false)?
                .span_to(&helpers::hour(0, false)?, false)?
                .latent()
                .form(Form::PartOfDay))
    );
    b.rule_1("evening",
        b.reg(r#"abends"#)?,
        |_| Ok(helpers::hour(18, false)?
                .span_to(&helpers::hour(0, false)?, false)?
                .form(Form::PartOfDay))
    );
    b.rule_1("late evening (latent)",
        b.reg(r#"sp[äa]te[nr] abend(?:stunden?)?|abend sp[äa]t"#)?,
        |_| Ok(helpers::hour(21, false)?
                .span_to(&helpers::hour(0, false)?, false)?
                .latent()
                .form(Form::PartOfDay))
    );
    b.rule_1("late evening",
        b.reg(r#"sp[äa]t(?: am )?abends?"#)?,
        |_| Ok(helpers::hour(21, false)?
                .span_to(&helpers::hour(0, false)?, false)?
                .form(Form::PartOfDay))
    );
    b.rule_1("early night (latent)",
        b.reg(r#"fr[üu]he[nr]? nachtstunden?"#)?,
        |_| Ok(helpers::hour(21, false)?
                .span_to(&helpers::hour(0, false)?, false)?
                .latent()
                .form(Form::PartOfDay))
    );
    b.rule_1("early night",
        b.reg(r#"fr[üu]h in der nacht"#)?,
        |_| Ok(helpers::hour(21, false)?
                .span_to(&helpers::hour(0, false)?, false)?
                .form(Form::PartOfDay))
    );
    b.rule_1("night (latent)",
        b.reg(r#"nacht"#)?,
        |_| Ok(helpers::hour(0, false)?
                .span_to(&helpers::hour(4, false)?, false)?
                .latent()
                .form(Form::PartOfDay))
    );
    b.rule_1("night",
        b.reg(r#"nachts"#)?,
        |_| Ok(helpers::hour(0, false)?
                .span_to(&helpers::hour(4, false)?, false)?
                .form(Form::PartOfDay))
    );
    b.rule_1("late night",
        b.reg(r#"(?:sp[äa]t|tief)(?: in der )?nachts?"#)?,
        |_| Ok(helpers::hour(0, false)?
                .span_to(&helpers::hour(4, false)?, false)?
                .form(Form::PartOfDay)) 
    );
    b.rule_2("in|during the <part-of-day>",
        b.reg(r#"(?:in|an|zu|beim?|am|um|w[äa]h?rend)(?: de(?:r|m|s|n)|die|das)?"#)?,
        time_check!(form!(Form::PartOfDay)),
        |_, time| Ok(time.value().clone().not_latent()) 
    );
    b.rule_2("this <part-of-day>",
        b.reg(r#"diesen?|dieses|heute"#)?,
        time_check!(form!(Form::PartOfDay)),
        |_, time| Ok(helpers::cycle_nth(Grain::Day, 0)?
                .intersect(time.value())?
                .form(Form::PartOfDay))
    );
    b.rule_1("tonight",
        b.reg(r#"heute? (?:am)? abends?"#)?,
        |_| Ok(helpers::cycle_nth(Grain::Day, 0)?
                .intersect(&helpers::hour(18, false)?
                            .span_to(&helpers::hour(0, false)?, false)?)?
                .form(Form::PartOfDay))
    );
    b.rule_1("after work",
        b.reg(r#"nach (?:der)? arbeit|(?:am)? feier ?abend"#)?,
        |_| Ok(helpers::cycle_nth(Grain::Day, 0)?
                .intersect(&helpers::hour(17, false)?
                            .span_to(&helpers::hour(21, false)?, false)?)?
                .form(Form::PartOfDay))
    );
    b.rule_2("<time> <part-of-day>",
        time_check!(),
        time_check!(form!(Form::PartOfDay)),
        |a, b| b.value().intersect(a.value())
    );
    b.rule_3("<part-of-day> of <time>",
        time_check!(form!(Form::PartOfDay)),
        b.reg(r#"des|von|vom|am"#)?,
        time_check!(),
        |a, _, b| a.value().intersect(b.value())
    );
    b.rule_1("week-end",
        b.reg(r#"wochen ?ende?"#)?,
        |_| {
            let friday = helpers::day_of_week(Weekday::Fri)?
                                .intersect(&helpers::hour(18, false)?)?;
            let monday = helpers::day_of_week(Weekday::Mon)?
                                .intersect(&helpers::hour(0, false)?)?;
            friday.span_to(&monday, false)
        }
    );
    b.rule_1("season",
        b.reg(r#"sommer"#)?,
        |_| helpers::month_day(6, 21)?.span_to(&helpers::month_day(9, 23)?, false)
    );
    b.rule_1("season",
        b.reg(r#"herbst"#)?,
        |_| helpers::month_day(9, 23)?.span_to(&helpers::month_day(12, 21)?, false)
    );
    b.rule_1("season",
        b.reg(r#"winter"#)?,
        |_| helpers::month_day(12, 21)?.span_to(&helpers::month_day(3, 20)?, false)
    );
    b.rule_1("season",
        b.reg(r#"fr[üu]hling|fr[üu]hjahr"#)?,
        |_| helpers::month_day(3, 20)?.span_to(&helpers::month_day(6, 21)?, false)
    );
    b.rule_2("<time-of-day> approximately",
        time_check!(form!(Form::TimeOfDay(_))),
        b.reg(r#"(?:um )?zirka|ungef[äa]hr|etwa"#)?,
        |time, _| Ok(time.value().clone().not_latent().precision(Approximate))
    );
    b.rule_2("<time-of-day> approximately",
        time_check!(form!(Form::TimeOfDay(_))),
        b.reg(r#"genau|exakt|p[üu]nktlich|punkt(?: um)?"#)?,
        |time, _| Ok(time.value().clone().not_latent().precision(Exact))
    );
    b.rule_2("about <time-of-day>",
        b.reg(r#"(?:um )?zirka|ungef[äa]hr|etwa"#)?,
        time_check!(form!(Form::TimeOfDay(_))),
        |_, time| Ok(time.value().clone().not_latent().precision(Approximate))
    );
    b.rule_2("exactly <time-of-day>",
        b.reg(r#"genau|exakt|p[üu]nktlich|punkt(?: um)?"#)?,
        time_check!(form!(Form::TimeOfDay(_))),
        |_, time| Ok(time.value().clone().not_latent().precision(Exact))
    );
    b.rule_4("<month> dd-dd (interval)",
        b.reg(r#"([012]?\d|30|31)(?:ter|\.)?"#)?,
        b.reg(r#"\-|bis"#)?,
        b.reg(r#"([012]?\d|30|31)(?:ter|\.)?"#)?,
        time_check!(form!(Form::Month(_))),
        |d1, _, d2, month| {
            let start = month.value()
                .intersect(&helpers::day_of_month(d1.group(1).parse()?)?)?;
            let end = month.value()
                .intersect(&helpers::day_of_month(d2.group(1).parse()?)?)?;
            start.span_to(&end, true)
        }
    );
    b.rule_3("<datetime> - <datetime> (interval)",
        time_check!(|time: &TimeValue| !time.latent),
        b.reg(r#"\-|bis"#)?,
        time_check!(|time: &TimeValue| !time.latent),
        |start, _, end| start.value().span_to(end.value(), true)
    );
    b.rule_4("between <datetime> and <datetime> (interval)",
        b.reg(r#"zwischen"#)?,
        time_check!(),
        b.reg(r#"und"#)?,
        time_check!(),
        |_, start, _, end| start.value().span_to(end.value(), true)
    );
    b.rule_3("<time-of-day> - <time-of-day> (interval)",
        time_check!(|time: &TimeValue| if let Form::TimeOfDay(_) = time.form { !time.latent } else { false }),
        b.reg(r#"\-|bis"#)?,
        time_check!(form!(Form::TimeOfDay(_))),
        |start, _, end| start.value().span_to(end.value(), true) 
    );
    b.rule_4("from <time-of-day> - <time-of-day> (interval)",
        b.reg(r#"(?:von|nach|ab|fr[üu]hestens (?:um)?)"#)?,
        time_check!(form!(Form::TimeOfDay(_))),
        b.reg(r#"(?:(?:noch|aber|jedoch)? vor)|\-|bis"#)?,
        time_check!(form!(Form::TimeOfDay(_))),
        |_, start, _, end| start.value().span_to(end.value(), true)
    );
    b.rule_4("between <time-of-day> and <time-of-day> (interval)",
        b.reg(r#"zwischen"#)?,
        time_check!(form!(Form::TimeOfDay(_))),
        b.reg(r#"und"#)?,
        time_check!(form!(Form::TimeOfDay(_))),
        |_, start, _, end| start.value().span_to(end.value(), true)
    );
    b.rule_2("within <duration>",
        b.reg(r#"binnen|innerhalb(?: von)?"#)?,
        duration_check!(),
        |_, duration| helpers::cycle_nth(Grain::Second, 0)?
            .span_to(&duration.value().in_present()?, false)
    );
    b.rule_2("by the end of <time>",
        b.reg(r#"bis (?:zum)? ende (?:von)?|(?:noch)? vor"#)?,
        time_check!(),
        |_, time| helpers::cycle_nth(Grain::Second, 0)?.span_to(time.value(), true)
    );
    b.rule_2("until <time-of-day>",
        b.reg(r#"vor|bis(?: zu[rm]?)?"#)?,
        time_check!(),
        |_, time| Ok(time.value().clone().direction(Some(Direction::Before)))
    );
    b.rule_2("after <time-of-day>",
        b.reg(r#"nach"#)?,
        time_check!(),
        |_, time| Ok(time.value().clone().direction(Some(Direction::After)))
    );
    Ok(())
}

pub fn rules_numbers(b:&mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect",
        number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
        number_check!(),
        |a, b| helpers::compose_numbers(&a.value(), &b.value()));
    b.rule_3("numbers und",
        integer_check!(1, 9),
        b.reg(r#"und"#)?,
        integer_check!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
        |a, _, b| IntegerValue::new(a.value().value + b.value().value)
    );
    b.rule_1("integer (0..19)",
        b.reg(r#"(kein(?:er|en|e?s?)|null|nichts|eins?(?:er|e)?|zwei|drei(?:zehn)?|vier(?:zehn)?|f[üu]nf(?:zehn)?|sech(?:s|zehn)|sieb(?:en|zehn)|acht(?:zehn)?|neun(?:zehn)?|elf|zw[öo]lf)"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                    "kein"     => 0,
                    "keine"    => 0, 
                    "keins"    => 0, 
                    "keines"   => 0, 
                    "keiner"   => 0, 
                    "keinen"   => 0, 
                    "null"     => 0, 
                    "nichts"   => 0,
                    "ein"      => 1, 
                    "eins"     => 1, 
                    "eine"     => 1, 
                    "einer"    => 1, 
                    "zwei"     => 2,
                    "drei"     => 3, 
                    "vier"     => 4, 
                    "fünf"     => 5,
                    "funf"     => 5,
                    "sechs"    => 6, 
                    "sieben"   => 7, 
                    "acht"     => 8, 
                    "neun"     => 9, 
                    "elf"      => 11,
                    "zwölf"    => 12,
                    "zwolf"    => 12,
                    "dreizehn" => 13,
                    "vierzehn" => 14,
                    "fünfzehn" => 15,
                    "funfzehn" => 15,
                    "sechzehn" => 16,
                    "siebzehn" => 17,
                    "achtzehn" => 18,
                    "neunzehn" => 19,
                _ => panic!("Unknown match {:?}", text_match.group(1)),
            };
            IntegerValue::new(value)
        }
    );
    b.rule_1("ten",
        b.reg(r#"zehn"#)?,
        |_| IntegerValue::new_with_grain(10, 1)
    );
    b.rule_1("dozen",
        b.reg(r#"dutzend"#)?,
        |_| Ok(IntegerValue {
                value: 12,
                grain: Some(1),
                group: true,
                ..IntegerValue::default()
            })
    );
    b.rule_1("hundred",
        b.reg(r#"hunderte?"#)?,
        |_| IntegerValue::new_with_grain(100, 2)
    );
    b.rule_1("thousand",
        b.reg(r#"tausende?"#)?,
        |_| IntegerValue::new_with_grain(1000, 3)
    );
    b.rule_1("million",
        b.reg(r#"million(?:en)?"#)?,
        |_| IntegerValue::new_with_grain(1000000, 6)
    );
    b.rule_1("couple",
        b.reg(r#"(?:ein )?paar"#)?,
        |_| IntegerValue::new(2)
    );
    b.rule_1("few",
        b.reg(r#"mehrere"#)?,
        |_| Ok(IntegerValue {
            value: 3,
            grain: Some(1),
            precision: Approximate,
            ..IntegerValue::default()
        })
    );
    b.rule_1("integer (20..90)",
        b.reg(r#"(zwanzig|dreissig|vierzig|f[üu]nfzig|sechzig|siebzig|achtzig|neunzig)"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "zwanzig"   => 20, 
                "dreissig"  => 30, 
                "vierzig"   => 40, 
                "funfzig"   => 50,
                "fünfzig"   => 50, 
                "sechzig"   => 60,
                "siebzig"   => 70, 
                "achtzig"   => 80, 
                "neunzig"   => 90,
                _ => panic!("Unknown match {:?} with a text match: {:?}", text_match.group(1), text_match),
            };
            IntegerValue::new_with_grain(value, 1)
        }
    );
    b.rule_1("integer ([2-9][1-9])",
        b.reg(r#"(ein|zwei|drei|vier|f[üu]nf|sechs|sieben|acht|neun)und(zwanzig|dreissig|vierzig|f[üu]nfzig|sechzig|siebzig|achtzig|neunzig)"#)?,
        |text_match| {
            let digit = match text_match.group(1).as_ref() {
                "ein"       => 1, 
                "zwei"      => 2, 
                "drei"      => 3, 
                "vier"      => 4, 
                "funf"      => 5,
                "fünf"      => 5,
                "sechs"     => 6, 
                "sieben"    => 7, 
                "acht"      => 8, 
                "neun"      => 9,
                _ => panic!("Unknown match {:?} with a text match: {:?}", text_match.group(1), text_match),
            };
            let tens_digit = match text_match.group(2).as_ref() {
                "zwanzig"  => 20, 
                "dreissig" => 30, 
                "vierzig"  => 40, 
                "funfzig"  => 50,
                "fünfzig"  => 50,
                "sechzig"  => 60, 
                "siebzig"  => 70, 
                "achtzig"  => 80, 
                "neunzig"  => 90,
                 _ => panic!("Unknown match {:?} with a text match: {:?}", text_match.group(2), text_match),

            };
            IntegerValue::new(digit + tens_digit)
        }
    );
    b.rule_1("integer (numeric)",
        b.reg(r#"(\d{1,18})"#)?,
        |text_match| IntegerValue::new(text_match.group(1).parse()?)
    );
    b.rule_1("integer with thousands separator .",
        b.reg(r#"(\d{1,3}(\.\d\d\d){1,5})"#)?,
        |text_match| IntegerValue::new(text_match.group(1).replace(".", "").parse()?)
    );
    
    b.rule_2("number hundreds",
        integer_check!(1, 99),
        integer_check!(100, 100),
        |a, b| Ok(IntegerValue {
            value: a.value().value * b.value().value,
            grain: b.value().grain,
            ..IntegerValue::default()
        })
    );

    b.rule_2("number thousands",
        integer_check!(1, 999),
        integer_check!(1000, 1000),
        |a, b| Ok(IntegerValue {
            value: a.value().value * b.value().value,
            grain: b.value().grain,
            ..IntegerValue::default()
        })
    );

    b.rule_2("number millions",
        integer_check!(1, 99),
        integer_check!(1000000, 1000000),
        |a, b| Ok(IntegerValue {
            value: a.value().value * b.value().value,
            grain: b.value().grain,
            ..IntegerValue::default()
        })
    );
    b.rule_1("decimal number",
        b.reg(r#"(\d*,\d+)"#)?,
        |text_match| FloatValue::new(text_match.group(1).replace(",", ".").parse()?)
    );
    b.rule_3("number dot number",
        number_check!(|number: &NumberValue| !number.prefixed()),
        b.reg(r#"komma"#)?,
        number_check!(|number: &NumberValue| !number.suffixed()),
        |a, _, b| FloatValue::new(b.value().value() * 0.1 + a.value().value())
    );
    b.rule_1("decimal with thousands separator",
        b.reg(r#"(\d+(\.\d\d\d)+,\d+)"#)?,
        |text_match| FloatValue::new(text_match.group(1).replace(".", "").replace(",", ".").parse()?)
    );
    b.rule_2("numbers prefix with -, negative or minus",
        b.reg(r#"-|minus|negativ"#)?,
        number_check!(|number: &NumberValue| !number.prefixed()),
        |_, a| -> RuleResult<NumberValue> {
            Ok(match a.value().clone() {
                   NumberValue::Integer(integer) => {
                       IntegerValue {
                               value: integer.value * -1,
                               prefixed: true,
                               ..integer
                           }
                           .into()
                   }
                   NumberValue::Float(float) => {
                       FloatValue {
                               value: float.value * -1.0,
                               prefixed: true,
                               ..float
                           }
                           .into()
                   }
            })
    });
    b.rule_2("numbers suffixes (K, M, G)",
        number_check!(|number: &NumberValue| !number.suffixed()),
        b.reg_neg_lh(r#"([kmg])"#, r#"^[^\W\$€]"#)?,
        |a, text_match| -> RuleResult<NumberValue> {
            let multiplier = match text_match.group(0).as_ref() {
                "k" => 1000,
                "m" => 1000000,
                "g" => 1000000000,
                _   => panic!("Unknown match"),
            };
            Ok(match a.value().clone() { // checked
                   NumberValue::Integer(integer) => {
                       IntegerValue {
                               value: integer.value * multiplier,
                               suffixed: true,
                               ..integer
                           }
                           .into()
                   }
                   NumberValue::Float(float) => {
                let product = float.value * (multiplier as f32);
                if product.floor() == product {
                    IntegerValue {
                            value: product as i64,
                            suffixed: true,
                            ..IntegerValue::default()
                        }
                        .into()
                } else {
                    FloatValue {
                            value: product,
                            suffixed: true,
                            ..float
                        }
                        .into()
                }
            }
        })
    });
    b.rule_1("ordinals (first..19th)",
        b.reg(r#"(erste|zweite|dritte|vierte|f[üu]nfte|sechste|siebte|achte|neunte|zehnte|elfte|zw[öo]lfte|dreizehnte|vierzehnte|f[üu]nfzehnte|sechzehnte|siebzehnte|achtzehnte|neunzehnte)(?:r|s|n|m)?"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "erste"       => 1, 
                "zweite"      => 2, 
                "dritte"      => 3, 
                "vierte"      => 4, 
                "funfte"      => 5,
                "fünfte"      => 5,
                "sechste"     => 6, 
                "siebte"      => 7, 
                "achte"       => 8, 
                "neunte"      => 9, 
                "zehnte"      => 10, 
                "elfte"       => 11,
                "zwölfte"     => 12,
                "zwolfte"     => 12, 
                "dreizehnte"  => 13,
                "vierzehnte"  => 14,
                "fünfzehnte"  => 15,
                "funfzehnte"  => 15, 
                "sechzehnte"  => 16,
                "siebzehnte"  => 17, 
                "achtzehnte"  => 18, 
                "neunzehnte"  => 19,
                _ => panic!("Unknown match {:?} with a text match: {:?}", text_match.group(1), text_match),
            };
            Ok(OrdinalValue { value: value })
        }
    );
    b.rule_1("ordinal (digits)",
        b.reg(r#"0*(\d+)(?:\.| ?(?:te(?:n|r|s)?)|(?:ste(?:n|r|s)?))"#)?,
        |text_match| Ok(OrdinalValue { value: text_match.group(1).parse()? })
    );
    b.rule_2("der <ordinal>",
        b.reg(r#"de(?:r|s|n|m)|das|die"#)?,
        ordinal_check!(),
        |_, ordinal| Ok(ordinal.value().clone())
    );
    b.rule_1("ordinal (20..90)",
        b.reg(r#"(zwanzigste|dreissigste|vierzigste|f[üu]nfzigste|sechzigste|siebzigste|achtzigste|neunzigste)(?:r|n|m|s)?"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "zwanzigste"   => 20, 
                "dreissigste"  => 30, 
                "vierzigste"   => 40, 
                "funfzigste"   => 50,
                "fünfzigste"   => 50, 
                "sechzigste"   => 60,
                "siebzigste"   => 70, 
                "achtzigste"   => 80, 
                "neunzigste"   => 90,
                _ => panic!("Unknown match {:?} with a text match: {:?}", text_match.group(1), text_match),
            };
            Ok(OrdinalValue { value })
        }
    );
    b.rule_1("ordinal ([2-9][1-9])",
        b.reg(r#"(ein|zwei|drei|vier|f[üu]nf|sechs|sieben|acht|neun)und(zwanzigste|dreissigste|vierzigste|f[üu]nfzigste|sechzigste|siebzigste|achtzigste|neunzigste)(?:r|n|m|s)?"#)?,
        |text_match| {
            let digit = match text_match.group(1).as_ref() {
                "ein"       => 1, 
                "zwei"      => 2, 
                "drei"      => 3, 
                "vier"      => 4, 
                "funf"      => 5,
                "fünf"      => 5,
                "sechs"     => 6, 
                "sieben"    => 7, 
                "acht"      => 8, 
                "neun"      => 9,
                _ => panic!("Unknown match {:?} with a text match: {:?}", text_match.group(1), text_match),
            };
            let tens_digit = match text_match.group(2).as_ref() {
                "zwanzigste"   => 20, 
                "dreissigste"  => 30, 
                "vierzigste"   => 40, 
                "funfzigste"   => 50,
                "fünfzigste"   => 50, 
                "sechzigste"   => 60,
                "siebzigste"   => 70, 
                "achtzigste"   => 80, 
                "neunzigste"   => 90,
                 _ => panic!("Unknown match {:?} with a text match: {:?}", text_match.group(2), text_match),

            };
            Ok(OrdinalValue { value: digit + tens_digit })
        }
    );
    Ok(())
}

// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//              ===> DO NOT CHANGE THIS FILE !!! <===                   +
// This file was automatically generated at: 2024-11-27T22:20:54.833    +
// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

use crate::model::*;
use crate::NR_OF_BOOKS_IN_TANACH;
use std::collections::HashMap;

pub fn get_tanach() -> Tanach {
    let mut tanach = Tanach {
        nr_of_books_expected: NR_OF_BOOKS_IN_TANACH,
        ..Default::default()
    };

    let book_meta_data = BookMetaData {
        uxlc_version: "1.9".to_string(),
        build_date: "4 Jul 2023  00:00".to_string(),
        build_version: "27.0".to_string(),
        nr_of_chapters: 2,
        nr_of_verses: 38,
        verses_per_chapter: HashMap::new(),
    };
    tanach.add_book("Haggai", &book_meta_data.clone());
    tanach.add_verses_per_chapter("Haggai".to_string(), 1, 15);

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 1,
        verse_no: 1,
    };
    tanach.add_verse(verse_ref, "בִּשְׁנַ֤ת שְׁתַּ֙יִם֙ לְדָרְיָ֣וֶשׁ הַמֶּ֔לֶךְ בַּחֹ֙דֶשׁ֙ הַשִּׁשִּׁ֔י בְּי֥וֹם אֶחָ֖ד לַחֹ֑דֶשׁ הָיָ֨ה דְבַר־יְהוָ֜ה בְּיַד־חַגַּ֣י הַנָּבִ֗יא אֶל־זְרֻבָּבֶ֤ל בֶּן־שְׁאַלְתִּיאֵל֙ פַּחַ֣ת יְהוּדָ֔ה וְאֶל־יְהוֹשֻׁ֧עַ בֶּן־יְהוֹצָדָ֛ק הַכֹּהֵ֥ן הַגָּד֖וֹל לֵאמֹֽר׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 1,
        verse_no: 2,
    };
    tanach.add_verse(
        verse_ref,
        "כֹּ֥ה אָמַ֛ר יְהוָ֥ה צְבָא֖וֹת לֵאמֹ֑ר הָעָ֤ם הַזֶּה֙ אָֽמְר֔וּ לֹ֥א עֶת־בֹּ֛א עֶת־בֵּ֥ית יְהוָ֖ה לְהִבָּנֽוֹת׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 1,
        verse_no: 3,
    };
    tanach.add_verse(verse_ref, "וַֽיְהִי֙ דְּבַר־יְהוָ֔ה בְּיַד־חַגַּ֥י הַנָּבִ֖יא לֵאמֹֽר׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 1,
        verse_no: 4,
    };
    tanach.add_verse(
        verse_ref,
        "הַעֵ֤ת לָכֶם֙ אַתֶּ֔ם לָשֶׁ֖בֶת בְּבָתֵּיכֶ֣ם סְפוּנִ֑ים וְהַבַּ֥יִת הַזֶּ֖ה חָרֵֽב׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 1,
        verse_no: 5,
    };
    tanach.add_verse(
        verse_ref,
        "וְעַתָּ֕ה כֹּ֥ה אָמַ֖ר יְהוָ֣ה צְבָא֑וֹת שִׂ֥ימוּ לְבַבְכֶ֖ם עַל־דַּרְכֵיכֶֽם׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 1,
        verse_no: 6,
    };
    tanach.add_verse(verse_ref, "זְרַעְתֶּ֨ם הַרְבֵּ֜ה וְהָבֵ֣א מְעָ֗ט אָכ֤וֹל וְאֵין־לְשָׂבְעָה֙ שָׁת֣וֹ וְאֵין־לְשָׁכְרָ֔ה לָב֖וֹשׁ וְאֵין־לְחֹ֣ם ל֑וֹ וְהַ֨מִּשְׂתַּכֵּ֔ר מִשְׂתַּכֵּ֖ר אֶל־צְר֥וֹר נָקֽוּב׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 1,
        verse_no: 7,
    };
    tanach.add_verse(
        verse_ref,
        "כֹּ֥ה אָמַ֖ר יְהוָ֣ה צְבָא֑וֹת שִׂ֥ימוּ לְבַבְכֶ֖ם עַל־דַּרְכֵיכֶֽם׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 1,
        verse_no: 8,
    };
    tanach.add_verse(
        verse_ref,
        "עֲל֥וּ הָהָ֛ר וַהֲבֵאתֶ֥ם עֵ֖ץ וּבְנ֣וּ הַבָּ֑יִת וְאֶרְצֶה־בּ֥וֹ *ואכבד **וְאֶכָּבְדָ֖ה אָמַ֥ר יְהוָֽה׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 1,
        verse_no: 9,
    };
    tanach.add_verse(verse_ref, "פָּנֹ֤ה אֶל־הַרְבֵּה֙ וְהִנֵּ֣ה לִמְעָ֔ט וַהֲבֵאתֶ֥ם הַבַּ֖יִת וְנָפַ֣חְתִּי ב֑וֹ יַ֣עַן מֶ֗ה נְאֻם֙ יְהוָ֣ה צְבָא֔וֹת יַ֗עַן בֵּיתִי֙ אֲשֶׁר־ה֣וּא חָרֵ֔ב וְאַתֶּ֥ם רָצִ֖ים אִ֥ישׁ לְבֵיתֽוֹ׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 1,
        verse_no: 10,
    };
    tanach.add_verse(
        verse_ref,
        "עַל־כֵּ֣ן עֲלֵיכֶ֔ם כָּלְא֥וּ שָמַ֖יִם מִטָּ֑ל וְהָאָ֖רֶץ כָּלְאָ֥ה יְבוּלָֽהּ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 1,
        verse_no: 11,
    };
    tanach.add_verse(verse_ref, "וָאֶקְרָ֨א חֹ֜רֶב עַל־הָאָ֣רֶץ וְעַל־הֶהָרִ֗ים וְעַל־הַדָּגָן֙ וְעַל־הַתִּיר֣וֹשׁ וְעַל־הַיִּצְהָ֔ר וְעַ֛ל אֲשֶׁ֥ר תּוֹצִ֖יא הָאֲדָמָ֑ה וְעַל־הָֽאָדָם֙ וְעַל־הַבְּהֵמָ֔ה וְעַ֖ל כָּל־יְגִ֥יעַ כַּפָּֽיִם׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 1,
        verse_no: 12,
    };
    tanach.add_verse(verse_ref, "וַיִּשְׁמַ֣ע זְרֻבָּבֶ֣ל ׀ בֶּֽן־שַׁלְתִּיאֵ֡ל וִיהוֹשֻׁ֣עַ בֶּן־יְהוֹצָדָק֩ הַכֹּהֵ֨ן הַגָּד֜וֹל וְכֹ֣ל ׀ שְׁאֵרִ֣ית הָעָ֗ם בְּקוֹל֙ יְהוָ֣ה אֱלֹֽהֵיהֶ֔ם וְעַל־דִּבְרֵי֙ חַגַּ֣י הַנָּבִ֔יא כַּאֲשֶׁ֥ר שְׁלָח֖וֹ יְהוָ֣ה אֱלֹהֵיהֶ֑ם וַיִּֽירְא֥וּ הָעָ֖ם מִפְּנֵ֥י יְהוָֽה׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 1,
        verse_no: 13,
    };
    tanach.add_verse(
        verse_ref,
        "וַ֠יֹּאמֶר חַגַּ֞י מַלְאַ֧ךְ יְהוָ֛ה בְּמַלְאֲכ֥וּת יְהוָ֖ה לָעָ֣ם לֵאמֹ֑ר אֲנִ֥י אִתְּכֶ֖ם נְאֻם־יְהוָֽה׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 1,
        verse_no: 14,
    };
    tanach.add_verse(verse_ref, "וַיָּ֣עַר יְהוָ֡ה אֶת־רוּחַ֩ זְרֻבָּבֶ֨ל בֶּן־שַׁלְתִּיאֵ֜ל פַּחַ֣ת יְהוּדָ֗ה וְאֶת־ר֙וּחַ֙ יְהוֹשֻׁ֤עַ בֶּן־יְהוֹצָדָק֙ הַכֹּהֵ֣ן הַגָּד֔וֹל וְֽאֶת־ר֔וּחַ כֹּ֖ל שְׁאֵרִ֣ית הָעָ֑ם וַיָּבֹ֙אוּ֙ וַיַּעֲשׂ֣וּ מְלָאכָ֔ה בְּבֵית־יְהוָ֥ה צְבָא֖וֹת אֱלֹהֵיהֶֽם׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 1,
        verse_no: 15,
    };
    tanach.add_verse(
        verse_ref,
        "בְּי֨וֹם עֶשְׂרִ֧ים וְאַרְבָּעָ֛ה לַחֹ֖דֶשׁ בַּשִּׁשִּׁ֑י בִּשְׁנַ֥ת שְׁתַּ֖יִם לְדָרְיָ֥וֶשׁ הַמֶּֽלֶךְ׃".to_string(),
    );
    tanach.add_verses_per_chapter("Haggai".to_string(), 2, 23);

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 1,
    };
    tanach.add_verse(
        verse_ref,
        "בַּשְּׁבִיעִ֕י בְּעֶשְׂרִ֥ים וְאֶחָ֖ד לַחֹ֑דֶשׁ הָיָה֙ דְּבַר־יְהוָ֔ה בְּיַד־חַגַּ֥י הַנָּבִ֖יא לֵאמֹֽר׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 2,
    };
    tanach.add_verse(
        verse_ref,
        "אֱמָר־נָ֗א אֶל־זְרֻבָּבֶ֤ל בֶּן־שַׁלְתִּיאֵל֙ פַּחַ֣ת יְהוּדָ֔ה וְאֶל־יְהוֹשֻׁ֥עַ בֶּן־יְהוֹצָדָ֖ק הַכֹּהֵ֣ן הַגָּד֑וֹל וְאֶל־שְׁאֵרִ֥ית הָעָ֖ם לֵאמֹֽר׃"
            .to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 3,
    };
    tanach.add_verse(verse_ref, "מִ֤י בָכֶם֙ הַנִּשְׁאָ֔ר אֲשֶׁ֤ר רָאָה֙ אֶת־הַבַּ֣יִת הַזֶּ֔ה בִּכְבוֹד֖וֹ הָרִאשׁ֑וֹן וּמָ֨ה אַתֶּ֜ם רֹאִ֤ים אֹתוֹ֙ עַ֔תָּה הֲל֥וֹא כָמֹ֛הוּ כְּאַ֖יִן בְּעֵינֵיכֶֽם׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 4,
    };
    tanach.add_verse(verse_ref, "וְעַתָּ֣ה חֲזַ֣ק זְרֻבָּבֶ֣ל ׀ נְאֻם־יְהוָ֡ה וַחֲזַ֣ק יְהוֹשֻׁ֣עַ בֶּן־יְהוֹצָדָק֩ הַכֹּהֵ֨ן הַגָּד֜וֹל וַחֲזַ֨ק כָּל־עַ֥ם הָאָ֛רֶץ נְאֻם־יְהוָ֖ה וַֽעֲשׂ֑וּ כִּֽי־אֲנִ֣י אִתְּכֶ֔ם נְאֻ֖ם יְהוָ֥ה צְבָאֽוֹת׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 5,
    };
    tanach.add_verse(
        verse_ref,
        "אֶֽת־הַדָּבָ֞ר אֲשֶׁר־כָּרַ֤תִּי אִתְּכֶם֙ בְּצֵאתְכֶ֣ם מִמִּצְרַ֔יִם וְרוּחִ֖י עֹמֶ֣דֶת בְּתוֹכְכֶ֑ם אַל־תִּירָֽאוּ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 6,
    };
    tanach.add_verse(
        verse_ref,
        "כִּ֣י כֹ֤ה אָמַר֙ יְהוָ֣ה צְבָא֔וֹת ע֥וֹד אַחַ֖ת מְעַ֣ט הִ֑יא וַאֲנִ֗י מַרְעִישׁ֙ אֶת־הַשָּׁמַ֣יִם וְאֶת־הָאָ֔רֶץ וְאֶת־הַיָּ֖ם וְאֶת־הֶחָרָבָֽה׃"
            .to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 7,
    };
    tanach.add_verse(
        verse_ref,
        "וְהִרְעַשְׁתִּי֙ אֶת־כָּל־הַגּוֹיִ֔ם וּבָ֖אוּ חֶמְדַּ֣ת כָּל־הַגּוֹיִ֑ם וּמִלֵּאתִ֞י אֶת־הַבַּ֤יִת הַזֶּה֙ כָּב֔וֹד אָמַ֖ר יְהוָ֥ה צְבָאֽוֹת׃"
            .to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 8,
    };
    tanach.add_verse(verse_ref, "לִ֥י הַכֶּ֖סֶף וְלִ֣י הַזָּהָ֑ב נְאֻ֖ם יְהוָ֥ה צְבָאֽוֹת׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 9,
    };
    tanach.add_verse(verse_ref, "גָּד֣וֹל יִֽהְיֶ֡ה כְּבוֹד֩ הַבַּ֨יִת הַזֶּ֤ה הָאַֽחֲרוֹן֙ מִן־הָ֣רִאשׁ֔וֹן אָמַ֖ר יְהוָ֣ה צְבָא֑וֹת וּבַמָּק֤וֹם הַזֶּה֙ אֶתֵּ֣ן שָׁל֔וֹם נְאֻ֖ם יְהוָ֥ה צְבָאֽוֹת׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 10,
    };
    tanach.add_verse(
        verse_ref,
        "בְּעֶשְׂרִ֤ים וְאַרְבָּעָה֙ לַתְּשִׁיעִ֔י בִּשְׁנַ֥ת שְׁתַּ֖יִם לְדָרְיָ֑וֶשׁ הָיָה֙ דְּבַר־יְהוָ֔ה אֶל־חַגַּ֥י הַנָּבִ֖יא לֵאמֹֽר׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 11,
    };
    tanach.add_verse(
        verse_ref,
        "כֹּ֥ה אָמַ֖ר יְהוָ֣ה צְבָא֑וֹת שְׁאַל־נָ֧א אֶת־הַכֹּהֲנִ֛ים תּוֹרָ֖ה לֵאמֹֽר׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 12,
    };
    tanach.add_verse(verse_ref, "הֵ֣ן ׀ יִשָּׂא־אִ֨ישׁ בְּשַׂר־קֹ֜דֶשׁ בִּכְנַ֣ף בִּגְד֗וֹ וְנָגַ֣ע בִּ֠כְנָפוֹ אֶל־הַלֶּ֨חֶם וְאֶל־הַנָּזִ֜יד וְאֶל־הַיַּ֧יִן וְאֶל־שֶׁ֛מֶן וְאֶל־כָּל־מַאֲכָ֖ל הֲיִקְדָּ֑שׁ וַיַּעֲנ֧וּ הַכֹּהֲנִ֛ים וַיֹּאמְר֖וּ לֹֽא׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 13,
    };
    tanach.add_verse(
        verse_ref,
        "וַיֹּ֣אמֶר חַגַּ֔י אִם־יִגַּ֧ע טְמֵא־נֶ֛פֶשׁ בְּכָל־אֵ֖לֶּה הֲיִטְמָ֑א וַיַּעֲנ֧וּ הַכֹּהֲנִ֛ים וַיֹּאמְר֖וּ יִטְמָֽא׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 14,
    };
    tanach.add_verse(verse_ref, "וַיַּ֨עַן חַגַּ֜י וַיֹּ֗אמֶר כֵּ֣ן הָֽעָם־הַ֠זֶּה וְכֵן־הַגּ֨וֹי הַזֶּ֤ה לְפָנַי֙ נְאֻם־יְהוָ֔ה וְכֵ֖ן כָּל־מַעֲשֵׂ֣ה יְדֵיהֶ֑ם וַאֲשֶׁ֥ר יַקְרִ֛יבוּ שָׁ֖ם טָמֵ֥א הֽוּא׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 15,
    };
    tanach.add_verse(
        verse_ref,
        "וְעַתָּה֙ שִֽׂימוּ־נָ֣א לְבַבְכֶ֔ם מִן־הַיּ֥וֹם הַזֶּ֖ה וָמָ֑עְלָה מִטֶּ֧רֶם שֽׂוּם־אֶ֛בֶן אֶל־אֶ֖בֶן בְּהֵיכַ֥ל יְהוָֽה׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 16,
    };
    tanach.add_verse(
        verse_ref,
        "מִֽהְיוֹתָ֥ם בָּא֙ אֶל־עֲרֵמַ֣ת עֶשְׂרִ֔ים וְהָיְתָ֖ה עֲשָׂרָ֑ה בָּ֣א אֶל־הַיֶּ֗קֶב לַחְשֹׂף֙ חֲמִשִּׁ֣ים פּוּרָ֔ה וְהָיְתָ֖ה עֶשְׂרִֽים׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 17,
    };
    tanach.add_verse(
        verse_ref,
        "הִכֵּ֨יתִי אֶתְכֶ֜ם בַּשִּׁדָּפ֤וֹן וּבַיֵּֽרָקוֹן֙ וּבַבָּרָ֔ד אֵ֖ת כָּל־מַעֲשֵׂ֣ה יְדֵיכֶ֑ם וְאֵין־אֶתְכֶ֥ם אֵלַ֖י נְאֻם־יְהוָֽה׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 18,
    };
    tanach.add_verse(verse_ref, "שִׂימוּ־נָ֣א לְבַבְכֶ֔ם מִן־הַיּ֥וֹם הַזֶּ֖ה וָמָ֑עְלָה מִיּוֹם֩ עֶשְׂרִ֨ים וְאַרְבָּעָ֜ה לַתְּשִׁיעִ֗י לְמִן־הַיּ֛וֹם אֲשֶׁר־יֻסַּ֥ד הֵֽיכַל־יְהוָ֖ה שִׂ֥ימוּ לְבַבְכֶֽם׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 19,
    };
    tanach.add_verse(
        verse_ref,
        "הַע֤וֹד הַזֶּ֙רַע֙ בַּמְּגוּרָ֔ה וְעַד־הַגֶּ֨פֶן וְהַתְּאֵנָ֧ה וְהָרִמּ֛וֹן וְעֵ֥ץ הַזַּ֖יִת לֹ֣א נָשָׂ֑א מִן־הַיּ֥וֹם הַזֶּ֖ה אֲבָרֵֽךְ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 20,
    };
    tanach.add_verse(
        verse_ref,
        "וַיְהִ֨י דְבַר־יְהוָ֤ה ׀ שֵׁנִית֙ אֶל־חַגַּ֔י בְּעֶשְׂרִ֧ים וְאַרְבָּעָ֛ה לַחֹ֖דֶשׁ לֵאמֹֽר׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 21,
    };
    tanach.add_verse(
        verse_ref,
        "אֱמֹ֕ר אֶל־זְרֻבָּבֶ֥ל פַּֽחַת־יְהוּדָ֖ה לֵאמֹ֑ר אֲנִ֣י מַרְעִ֔ישׁ אֶת־הַשָּׁמַ֖יִם וְאֶת־הָאָֽרֶץ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 22,
    };
    tanach.add_verse(verse_ref, "וְהָֽפַכְתִּי֙ כִּסֵּ֣א מַמְלָכ֔וֹת וְהִ֨שְׁמַדְתִּ֔י חֹ֖זֶק מַמְלְכ֣וֹת הַגּוֹיִ֑ם וְהָפַכְתִּ֤י מֶרְכָּבָה֙ וְרֹ֣כְבֶ֔יהָ וְיָרְד֤וּ סוּסִים֙ וְרֹ֣כְבֵיהֶ֔ם אִ֖ישׁ בְּחֶ֥רֶב אָחִֽיו׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Haggai".to_string(),
        chapter_no: 2,
        verse_no: 23,
    };
    tanach.add_verse(verse_ref, "בַּיּ֣וֹם הַה֣וּא נְאֻם־יְהוָ֣ה צְבָא֡וֹת אֶ֠קָּחֲךָ זְרֻבָּבֶ֨ל בֶּן־שְׁאַלְתִּיאֵ֤ל עַבְדִּי֙ נְאֻם־יְהוָ֔ה וְשַׂמְתִּ֖יךָ כַּֽחוֹתָ֑ם כִּֽי־בְךָ֣ בָחַ֔רְתִּי נְאֻ֖ם יְהוָ֥ה צְבָאֽוֹת׃".to_string());
    let book_meta_data = BookMetaData {
        uxlc_version: "1.9".to_string(),
        build_date: "4 Jul 2023  00:00".to_string(),
        build_version: "27.0".to_string(),
        nr_of_chapters: 1,
        nr_of_verses: 21,
        verses_per_chapter: HashMap::new(),
    };
    tanach.add_book("Obadiah", &book_meta_data.clone());
    tanach.add_verses_per_chapter("Obadiah".to_string(), 1, 21);

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 1,
    };
    tanach.add_verse(verse_ref, "חֲז֖וֹן עֹֽבַדְיָ֑ה כֹּֽה־אָמַר֩ אֲדֹנָ֨י יְהוִ֜ה לֶאֱד֗וֹם שְׁמוּעָ֨ה שָׁמַ֜עְנוּ מֵאֵ֤ת יְהוָה֙ וְצִיר֙ בַּגּוֹיִ֣ם שֻׁלָּ֔ח ק֛וּמוּ וְנָק֥וּמָה עָלֶיהָ לַמִּלְחָמָֽה׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 2,
    };
    tanach.add_verse(verse_ref, "הִנֵּ֥ה קָטֹ֛ן נְתַתִּ֖יךָ בַּגּוֹיִ֑ם בָּז֥וּי אַתָּ֖ה מְאֹֽד׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 3,
    };
    tanach.add_verse(
        verse_ref,
        "זְד֤וֹן לִבְּךָ֙ הִשִּׁיאֶ֔ךָ שֹׁכְנִ֥י בְחַגְוֵי־סֶ֖לַע מְר֣וֹם שִׁבְתּ֑וֹ אֹמֵ֣ר בְּלִבּ֔וֹ מִ֥י יוֹרִדֵ֖נִי אָֽרֶץ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 4,
    };
    tanach.add_verse(
        verse_ref,
        "אִם־תַּגְבִּ֣יהַּ כַּנֶּ֔שֶׁר וְאִם־בֵּ֥ין כּֽוֹכָבִ֖ים שִׂ֣ים קִנֶּ֑ךָ מִשָּׁ֥ם אוֹרִֽידְךָ֖ נְאֻם־יְהוָֽה׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 5,
    };
    tanach.add_verse(verse_ref, "אִם־גַּנָּבִ֤ים בָּאֽוּ־לְךָ֙ אִם־שׁ֣וֹדְדֵי לַ֔יְלָה אֵ֣יךְ נִדְמֵ֔יתָה הֲל֥וֹא יִגְנְב֖וּ דַּיָּ֑ם אִם־בֹּֽצְרִים֙ בָּ֣אוּ לָ֔ךְ הֲל֖וֹא יַשְׁאִ֥ירוּ עֹלֵלֽוֹת׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 6,
    };
    tanach.add_verse(verse_ref, "אֵ֚יךְ נֶחְפְּשׂ֣וּ עֵשָׂ֔ו נִבְע֖וּ מַצְפֻּנָֽיו׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 7,
    };
    tanach.add_verse(
        verse_ref,
        "עַֽד־הַגְּב֣וּל שִׁלְּח֗וּךָ כֹּ֚ל אַנְשֵׁ֣י בְרִיתֶ֔ךָ הִשִּׁיא֛וּךָ יָכְל֥וּ לְךָ֖ אַנְשֵׁ֣י שְׁלֹמֶ֑ךָ לַחְמְךָ֗ יָשִׂ֤ימוּ מָזוֹר֙ תַּחְתֶּ֔יךָ אֵ֥ין תְּבוּנָ֖ה בּֽוֹ׃"
            .to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 8,
    };
    tanach.add_verse(
        verse_ref,
        "הֲל֛וֹא בַּיּ֥וֹם הַה֖וּא נְאֻם יְהוָ֑ה וְהַאֲבַדְתִּ֤י חֲכָמִים֙ מֵֽאֱד֔וֹם וּתְבוּנָ֖ה מֵהַ֥ר עֵשָֽׂו׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 9,
    };
    tanach.add_verse(
        verse_ref,
        "וְחַתּ֥וּ גִבּוֹרֶ֖יךָ תֵּימָ֑ן לְמַ֧עַן יִכָּֽרֶת־אִ֛ישׁ מֵהַ֥ר עֵשָׂ֖ו מִקָּֽטֶל׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 10,
    };
    tanach.add_verse(
        verse_ref,
        "מֵחֲמַ֛ס אָחִ֥יךָ יַעֲקֹ֖ב תְּכַסְּךָ֣ בוּשָׁ֑ה וְנִכְרַ֖תָּ לְעוֹלָֽם׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 11,
    };
    tanach.add_verse(verse_ref, "בְּיוֹם֙ עֲמָֽדְךָ֣ מִנֶּ֔גֶד בְּי֛וֹם שְׁב֥וֹת זָרִ֖ים חֵיל֑וֹ וְנָכְרִ֞ים בָּ֣אוּ *שערו **שְׁעָרָ֗יו וְעַל־יְרוּשָׁלִַ֙ם֙ יַדּ֣וּ גוֹרָ֔ל גַּם־אַתָּ֖ה כְּאַחַ֥ד מֵהֶֽם׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 12,
    };
    tanach.add_verse(
        verse_ref,
        "וְאַל־תֵּ֤רֶא בְיוֹם־אָחִ֙יךָ֙ בְּי֣וֹם נָכְר֔וֹ וְאַל־תִּשְׂמַ֥ח לִבְנֵֽי יְהוּדָ֖ה בְּי֣וֹם אָבְדָ֑ם וְאַל־תַּגְדֵּ֥ל פִּ֖יךָ בְּי֥וֹם צָרָֽה׃"
            .to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 13,
    };
    tanach.add_verse(
        verse_ref,
        "אַל־תָּב֤וֹא בְשַֽׁעַר־עַמִּי֙ בְּי֣וֹם אֵידָ֔ם אַל־תֵּ֧רֶא גַם־אַתָּ֛ה בְּרָעָת֖וֹ בְּי֣וֹם אֵיד֑וֹ וְאַל־תִּשְׁלַ֥חְנָה בְחֵיל֖וֹ בְּי֥וֹם אֵידֽוֹ׃"
            .to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 14,
    };
    tanach.add_verse(
        verse_ref,
        "וְאַֽל־תַּעֲמֹד֙ עַל־הַפֶּ֔רֶק לְהַכְרִ֖ית אֶת־פְּלִיטָ֑יו וְאַל־תַּסְגֵּ֥ר שְׂרִידָ֖יו בְּי֥וֹם צָרָֽה׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 15,
    };
    tanach.add_verse(
        verse_ref,
        "כִּֽי־קָר֥וֹב יוֹם־יְהוָ֖ה עַל־כָּל־הַגּוֹיִ֑ם כַּאֲשֶׁ֤ר עָשִׂ֙יתָ֙ יֵעָ֣שֶׂה לָּ֔ךְ גְּמֻלְךָ֖ יָשׁ֥וּב בְּרֹאשֶֽׁךָ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 16,
    };
    tanach.add_verse(
        verse_ref,
        "כִּ֗י כַּֽאֲשֶׁ֤ר שְׁתִיתֶם֙ עַל־הַ֣ר קָדְשִׁ֔י יִשְׁתּ֥וּ כָֽל־הַגּוֹיִ֖ם תָּמִ֑יד וְשָׁת֣וּ וְלָע֔וּ וְהָי֖וּ כְּל֥וֹא הָֽיוּ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 17,
    };
    tanach.add_verse(
        verse_ref,
        "וּבְהַ֥ר צִיּ֛וֹן תִּהְיֶ֥ה פְלֵיטָ֖ה וְהָ֣יָה קֹ֑דֶשׁ וְיָֽרְשׁוּ֙ בֵּ֣ית יַֽעֲקֹ֔ב אֵ֖ת מוֹרָֽשֵׁיהֶם׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 18,
    };
    tanach.add_verse(verse_ref, "וְהָיָה֩ בֵית־יַעֲקֹ֨ב אֵ֜שׁ וּבֵ֧ית יוֹסֵ֣ף לֶהָבָ֗ה וּבֵ֤ית עֵשָׂו֙ לְקַ֔שׁ וְדָלְק֥וּ בָהֶ֖ם וַאֲכָל֑וּם וְלֹֽא־יִֽהְיֶ֤ה שָׂרִיד֙ לְבֵ֣ית עֵשָׂ֔ו כִּ֥י יְהוָ֖ה דִּבֵּֽר׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 19,
    };
    tanach.add_verse(
        verse_ref,
        "וְיָרְשׁ֨וּ הַנֶּ֜גֶב אֶת־הַ֣ר עֵשָׂ֗ו וְהַשְּׁפֵלָה֙ אֶת־פְּלִשְׁתִּ֔ים וְיָרְשׁוּ֙ אֶת־שְׂדֵ֣ה אֶפְרַ֔יִם וְאֵ֖ת שְׂדֵ֣ה שֹׁמְר֑וֹן וּבִנְיָמִ֖ן אֶת־הַגִּלְעָֽד׃"
            .to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 20,
    };
    tanach.add_verse(
        verse_ref,
        "וְגָלֻ֣ת הַֽחֵל־הַ֠זֶּה לִבְנֵ֨י יִשְׂרָאֵ֤ל אֲשֶֽׁר־כְּנַעֲנִים֙ עַד־צָ֣רְפַ֔ת וְגָלֻ֥ת יְרוּשָׁלַ֖͏ִם אֲשֶׁ֣ר בִּסְפָרַ֑ד יִֽרְשׁ֕וּ אֵ֖ת עָרֵ֥י הַנֶּֽגֶב׃"
            .to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Obadiah".to_string(),
        chapter_no: 1,
        verse_no: 21,
    };
    tanach.add_verse(
        verse_ref,
        "וְעָל֤וּ מֽוֹשִׁעִים֙ בְּהַ֣ר צִיּ֔וֹן לִשְׁפֹּ֖ט אֶת־הַ֣ר עֵשָׂ֑ו וְהָיְתָ֥ה לַֽיהוָ֖ה הַמְּלוּכָֽה׃".to_string(),
    );
    let book_meta_data = BookMetaData {
        uxlc_version: "1.9".to_string(),
        build_date: "4 Jul 2023  00:00".to_string(),
        build_version: "27.0".to_string(),
        nr_of_chapters: 3,
        nr_of_verses: 47,
        verses_per_chapter: HashMap::new(),
    };
    tanach.add_book("Nahum", &book_meta_data.clone());
    tanach.add_verses_per_chapter("Nahum".to_string(), 1, 14);

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 1,
        verse_no: 1,
    };
    tanach.add_verse(verse_ref, "מַשָּׂ֖א נִֽינְוֵ֑ה סֵ֧פֶר חֲז֛וֹן נַח֖וּם הָאֶלְקֹשִֽׁי׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 1,
        verse_no: 2,
    };
    tanach.add_verse(
        verse_ref,
        "אֵ֣ל קַנּ֤וֹא וְנֹקֵם֙ יְהוָ֔ה נֹקֵ֥ם יְהוָ֖ה וּבַ֣עַל חֵמָ֑ה נֹקֵ֤ם יְהוָה֙ לְצָרָ֔יו וְנוֹטֵ֥ר ה֖וּא לְאֹיְבָֽיו׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 1,
        verse_no: 3,
    };
    tanach.add_verse(
        verse_ref,
        "יְהֹוָ֗ה אֶ֤רֶךְ אַפַּ֙יִם֙ *וגדול־**וּגְדָל־כֹּ֔חַ וְנַקֵּ֖ה לֹ֣א יְנַקֶּ֑ה יְהוָ֗ה בְּסוּפָ֤ה וּבִשְׂעָרָה֙ דַּרְכּ֔וֹ וְעָנָ֖ן אֲבַ֥ק רַגְלָֽיו׃"
            .to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 1,
        verse_no: 4,
    };
    tanach.add_verse(
        verse_ref,
        "גּוֹעֵ֤ר בַּיָּם֙ וַֽיַּבְּשֵׁ֔הוּ וְכָל־הַנְּהָר֖וֹת הֶֽחֱרִ֑יב אֻמְלַ֤ל בָּשָׁן֙ וְכַרְמֶ֔ל וּפֶ֥רַח לְבָנ֖וֹן אֻמְלָֽל׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 1,
        verse_no: 5,
    };
    tanach.add_verse(
        verse_ref,
        "הָרִים֙ רָעֲשׁ֣וּ מִמֶּ֔נּוּ וְהַגְּבָע֖וֹת הִתְמֹגָ֑גוּ וַתִּשָּׂ֤א הָאָ֙רֶץ֙ מִפָּנָ֔יו וְתֵבֵ֖ל וְכָל־יֹ֥שְׁבֵי בָֽהּ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 1,
        verse_no: 6,
    };
    tanach.add_verse(
        verse_ref,
        "לִפְנֵ֤י זַעְמוֹ֙ מִ֣י יַֽעֲמ֔וֹד וּמִ֥י יָק֖וּם בַּחֲר֣וֹן אַפּ֑וֹ חֲמָתוֹ֙ נִתְּכָ֣ה כָאֵ֔שׁ וְהַצֻּרִ֖ים נִתְּצ֥וּ מִמֶּֽנּוּ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 1,
        verse_no: 7,
    };
    tanach.add_verse(
        verse_ref,
        "ט֣וֹב יְהוָ֔ה לְמָע֖וֹז בְּי֣וֹם צָרָ֑ה וְיֹדֵ֖עַ חֹ֥סֵי בֽוֹ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 1,
        verse_no: 8,
    };
    tanach.add_verse(
        verse_ref,
        "וּבְשֶׁ֣טֶף עֹבֵ֔ר כָּלָ֖ה יַעֲשֶׂ֣ה מְקוֹמָ֑הּ וְאֹיְבָ֖יו יְרַדֶּף־חֹֽשֶׁךְ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 1,
        verse_no: 9,
    };
    tanach.add_verse(
        verse_ref,
        "מַה־תְּחַשְּׁבוּן֙ אֶל־יְהוָ֔ה כָּלָ֖ה ה֣וּא עֹשֶׂ֑ה לֹֽא־תָק֥וּם פַּעֲמַ֖יִם צָרָֽה׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 1,
        verse_no: 10,
    };
    tanach.add_verse(
        verse_ref,
        "כִּ֚י עַד־סִירִ֣ים סְבֻכִ֔ים וּכְסָבְאָ֖ם סְבוּאִ֑ים אֻ֨כְּל֔וּ כְּקַ֥שׁ יָבֵ֖שׁ מָלֵֽא׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 1,
        verse_no: 11,
    };
    tanach.add_verse(verse_ref, "מִמֵּ֣ךְ יָצָ֔א חֹשֵׁ֥ב עַל־יְהוָ֖ה רָעָ֑ה יֹעֵ֖ץ בְּלִיָּֽעַל׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 1,
        verse_no: 12,
    };
    tanach.add_verse(
        verse_ref,
        "כֹּ֣ה ׀ אָמַ֣ר יְהוָ֗ה אִם־שְׁלֵמִים֙ וְכֵ֣ן רַבִּ֔ים וְכֵ֥ן נָגֹ֖זּוּ וְעָבָ֑ר וְעִ֨נִּתִ֔ךְ לֹ֥א אֲעַנֵּ֖ךְ עֽוֹד׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 1,
        verse_no: 13,
    };
    tanach.add_verse(verse_ref, "וְעַתָּ֕ה אֶשְׁבֹּ֥ר מֹטֵ֖הוּ מֵֽעָלָ֑יִךְ וּמוֹסְרֹתַ֖יִךְ אֲנַתֵּֽק׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 1,
        verse_no: 14,
    };
    tanach.add_verse(
        verse_ref,
        "וְצִוָּ֤ה עָלֶ֙יךָ֙ יְהוָ֔ה לֹֽא־יִזָּרַ֥ע מִשִּׁמְךָ֖ ע֑וֹד מִבֵּ֨ית אֱלֹהֶ֜יךָ אַכְרִ֨ית פֶּ֧סֶל וּמַסֵּכָ֛ה אָשִׂ֥ים קִבְרֶ֖ךָ כִּ֥י קַלּֽוֹתָ׃".to_string(),
    );
    tanach.add_verses_per_chapter("Nahum".to_string(), 2, 14);

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 2,
        verse_no: 1,
    };
    tanach.add_verse(verse_ref, "הִנֵּ֨ה עַל־הֶהָרִ֜ים רַגְלֵ֤י מְבַשֵּׂר֙ מַשְׁמִ֣יעַ שָׁל֔וֹם חָגִּ֧י יְהוּדָ֛ה חַגַּ֖יִךְ שַׁלְּמִ֣י נְדָרָ֑יִךְ כִּי֩ לֹ֨א יוֹסִ֥יף ע֛וֹד *לעבור־**לַֽעֲבָר־בָּ֥ךְ בְּלִיַּ֖עַל כֻּלֹּ֥ה נִכְרָֽת׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 2,
        verse_no: 2,
    };
    tanach.add_verse(
        verse_ref,
        "עָלָ֥ה מֵפִ֛יץ עַל־פָּנַ֖יִךְ נָצ֣וֹר מְצֻרָ֑ה צַפֵּה־דֶ֙רֶךְ֙ חַזֵּ֣ק מָתְנַ֔יִם אַמֵּ֥ץ כֹּ֖חַ מְאֹֽד׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 2,
        verse_no: 3,
    };
    tanach.add_verse(
        verse_ref,
        "כִּ֣י שָׁ֤ב יְהוָה֙ אֶת־גְּא֣וֹן יַעֲקֹ֔ב כִּגְא֖וֹן יִשְׂרָאֵ֑ל כִּ֤י בְקָקוּם֙ בֹּֽקְקִ֔ים וּזְמֹרֵיהֶ֖ם שִׁחֵֽתוּ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 2,
        verse_no: 4,
    };
    tanach.add_verse(
        verse_ref,
        "מָגֵ֨ן גִּבֹּרֵ֜יהוּ מְאָדָּ֗ם אַנְשֵׁי־חַ֙יִל֙ מְתֻלָּעִ֔ים בְּאֵשׁ־פְּלָד֥וֹת הָרֶ֖כֶב בְּי֣וֹם הֲכִינ֑וֹ וְהַבְּרֹשִׁ֖ים הָרְעָֽלוּ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 2,
        verse_no: 5,
    };
    tanach.add_verse(
        verse_ref,
        "בַּֽחוּצוֹת֙ יִתְהוֹלְל֣וּ הָרֶ֔כֶב יִֽשְׁתַּקְשְׁק֖וּן בָּרְחֹב֑וֹת מַרְאֵיהֶן֙ כַּלַּפִּידִ֔ם כַּבְּרָקִ֖ים יְרוֹצֵֽצוּ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 2,
        verse_no: 6,
    };
    tanach.add_verse(
        verse_ref,
        "יִזְכֹּר֙ אַדִּירָ֔יו יִכָּשְׁל֖וּ *בהלכותם **בַּהֲלִֽיכָתָ֑ם יְמַֽהֲרוּ֙ חֽוֹמָתָ֔הּ וְהֻכַ֖ן הַסֹּכֵֽךְ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 2,
        verse_no: 7,
    };
    tanach.add_verse(verse_ref, "שַׁעֲרֵ֥י הַנְּהָר֖וֹת נִפְתָּ֑חוּ וְהַֽהֵיכָ֖ל נָמֽוֹג׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 2,
        verse_no: 8,
    };
    tanach.add_verse(
        verse_ref,
        "וְהֻצַּ֖ב גֻּלְּתָ֣ה הֹֽעֲלָ֑תָה וְאַמְהֹתֶ֗יהָ מְנַֽהֲגוֹת֙ כְּק֣וֹל יוֹנִ֔ים מְתֹפְפֹ֖ת עַל־לִבְבֵהֶֽן׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 2,
        verse_no: 9,
    };
    tanach.add_verse(
        verse_ref,
        "וְנִינְוֵ֥ה כִבְרֵֽכַת־מַ֖יִם מִ֣ימֵי הִ֑יא וְהֵ֣מָּה נָסִ֔ים עִמְד֥וּ עֲמֹ֖דוּ וְאֵ֥ין מַפְנֶֽה׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 2,
        verse_no: 10,
    };
    tanach.add_verse(
        verse_ref,
        "בֹּ֥זּוּ כֶ֖סֶף בֹּ֣זּוּ זָהָ֑ב וְאֵ֥ין קֵ֙צֶה֙ לַתְּכוּנָ֔ה כָּבֹ֕ד מִכֹּ֖ל כְּלִ֥י חֶמְדָּֽה׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 2,
        verse_no: 11,
    };
    tanach.add_verse(
        verse_ref,
        "בּוּקָ֥ה וּמְבוּקָ֖ה וּמְבֻלָּקָ֑ה וְלֵ֨ב נָמֵ֜ס וּפִ֣ק בִּרְכַּ֗יִם וְחַלְחָלָה֙ בְּכָל־מָתְנַ֔יִם וּפְנֵ֥י כֻלָּ֖ם קִבְּצ֥וּ פָארֽוּר׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 2,
        verse_no: 12,
    };
    tanach.add_verse(
        verse_ref,
        "אַיֵּה֙ מְע֣וֹן אֲרָי֔וֹת וּמִרְעֶ֥ה ה֖וּא לַכְּפִרִ֑ים אֲשֶׁ֣ר הָלַךְ֩ אַרְיֵ֨ה לָבִ֥יא שָׁ֛ם גּ֥וּר אַרְיֵ֖ה וְאֵ֥ין מַחֲרִֽיד׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 2,
        verse_no: 13,
    };
    tanach.add_verse(
        verse_ref,
        "אַרְיֵ֤ה טֹרֵף֙ בְּדֵ֣י גֹֽרוֹתָ֔יו וּמְחַנֵּ֖ק לְלִבְאֹתָ֑יו וַיְמַלֵּא־טֶ֣רֶף חֹרָ֔יו וּמְעֹֽנֹתָ֖יו טְרֵפָֽה׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 2,
        verse_no: 14,
    };
    tanach.add_verse(verse_ref, "הִנְנִ֣י אֵלַ֗יִךְ נְאֻם֙ יְהוָ֣ה צְבָא֔וֹת וְהִבְעַרְתִּ֤י בֶֽעָשָׁן֙ רִכְבָּ֔הּ וּכְפִירַ֖יִךְ תֹּ֣אכַל חָ֑רֶב וְהִכְרַתִּ֤י מֵאֶ֙רֶץ֙ טַרְפֵּ֔ךְ וְלֹֽא־יִשָּׁמַ֥ע ע֖וֹד ק֥וֹל מַלְאָכֵֽכֵה׃".to_string());
    tanach.add_verses_per_chapter("Nahum".to_string(), 3, 19);

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 1,
    };
    tanach.add_verse(
        verse_ref,
        "ה֖וֹי עִ֣יר דָּמִ֑ים כֻּלָּ֗הּ כַּ֤חַשׁ פֶּ֙רֶק֙ מְלֵאָ֔ה לֹ֥א יָמִ֖ישׁ טָֽרֶף׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 2,
    };
    tanach.add_verse(
        verse_ref,
        "ק֣וֹל שׁ֔וֹט וְק֖וֹל רַ֣עַשׁ אוֹפָ֑ן וְס֣וּס דֹּהֵ֔ר וּמֶרְכָּבָ֖ה מְרַקֵּדָֽה׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 3,
    };
    tanach.add_verse(
        verse_ref,
        "פָּרָ֣שׁ מַעֲלֶ֗ה וְלַ֤הַב חֶ֙רֶב֙ וּבְרַ֣ק חֲנִ֔ית וְרֹ֥ב חָלָ֖ל וְכֹ֣בֶד פָּ֑גֶר וְאֵ֥ין קֵ֙צֶה֙ לַגְּוִיָּ֔ה *יכשלו **וְכָשְׁל֖וּ בִּגְוִיָּתָֽם׃"
            .to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 4,
    };
    tanach.add_verse(
        verse_ref,
        "מֵרֹב֙ זְנוּנֵ֣י זוֹנָ֔ה ט֥וֹבַת חֵ֖ן בַּעֲלַ֣ת כְּשָׁפִ֑ים הַמֹּכֶ֤רֶת גּוֹיִם֙ בִּזְנוּנֶ֔יהָ וּמִשְׁפָּח֖וֹת בִּכְשָׁפֶֽיהָ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 5,
    };
    tanach.add_verse(
        verse_ref,
        "הִנְנִ֣י אֵלַ֗יִךְ נְאֻם֙ יְהוָ֣ה צְבָא֔וֹת וְגִלֵּיתִ֥י שׁוּלַ֖יִךְ עַל־פָּנָ֑יִךְ וְהַרְאֵיתִ֤י גוֹיִם֙ מַעְרֵ֔ךְ וּמַמְלָכ֖וֹת קְלוֹנֵֽךְ׃"
            .to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 6,
    };
    tanach.add_verse(
        verse_ref,
        "וְהִשְׁלַכְתִּ֥י עָלַ֛יִךְ שִׁקֻּצִ֖ים וְנִבַּלְתִּ֑יךְ וְשַׂמְתִּ֖יךְ כְּרֹֽאִי׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 7,
    };
    tanach.add_verse(
        verse_ref,
        "וְהָיָ֤ה כָל־רֹאַ֙יִךְ֙ יִדּ֣וֹד מִמֵּ֔ךְ וְאָמַר֙ שָׁדְּדָ֣ה נִֽינְוֵ֔ה מִ֖י יָנ֣וּד לָ֑הּ מֵאַ֛יִן אֲבַקֵּ֥שׁ מְנַחֲמִ֖ים לָֽךְ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 8,
    };
    tanach.add_verse(
        verse_ref,
        "הֲתֵֽיטְבִי֙ מִנֹּ֣א אָמ֔וֹן הַיֹּֽשְׁבָה֙ בַּיְאֹרִ֔ים מַ֖יִם סָבִ֣יב לָ֑הּ אֲשֶׁר־חֵ֣יל יָ֔ם מִיָּ֖ם חוֹמָתָֽהּ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 9,
    };
    tanach.add_verse(
        verse_ref,
        "כּ֥וּשׁ עָצְמָ֛ה וּמִצְרַ֖יִם וְאֵ֣ין קֵ֑צֶה פּ֣וּט וְלוּבִ֔ים הָי֖וּ בְּעֶזְרָתֵֽךְ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 10,
    };
    tanach.add_verse(verse_ref, "גַּם־הִ֗יא לַגֹּלָה֙ הָלְכָ֣ה בַשֶּׁ֔בִי גַּ֧ם עֹלָלֶ֛יהָ יְרֻטְּשׁ֖וּ בְּרֹ֣אשׁ כָּל־חוּצ֑וֹת וְעַל־נִכְבַּדֶּ֙יהָ֙ יַדּ֣וּ גוֹרָ֔ל וְכָל־גְּדוֹלֶ֖יהָ רֻתְּק֥וּ בַזִּקִּֽים׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 11,
    };
    tanach.add_verse(
        verse_ref,
        "גַּם־אַ֣תְּ תִּשְׁכְּרִ֔י תְּהִ֖י נַֽעֲלָמָ֑ה גַּם־אַ֛תְּ תְּבַקְשִׁ֥י מָע֖וֹז מֵאוֹיֵֽב׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 12,
    };
    tanach.add_verse(
        verse_ref,
        "כָּ֨ל־מִבְצָרַ֔יִךְ תְּאֵנִ֖ים עִם־בִּכּוּרִ֑ים אִם־יִנּ֕וֹעוּ וְנָפְל֖וּ עַל־פִּ֥י אוֹכֵֽל׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 13,
    };
    tanach.add_verse(
        verse_ref,
        "הִנֵּ֨ה עַמֵּ֤ךְ נָשִׁים֙ בְּקִרְבֵּ֔ךְ לְאֹ֣יְבַ֔יִךְ פָּת֥וֹחַ נִפְתְּח֖וּ שַׁעֲרֵ֣י אַרְצֵ֑ךְ אָכְלָ֥ה אֵ֖שׁ בְּרִיחָֽיִך׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 14,
    };
    tanach.add_verse(
        verse_ref,
        "מֵ֤י מָצוֹר֙ שַֽׁאֲבִי־לָ֔ךְ חַזְּקִ֖י מִבְצָרָ֑יִךְ בֹּ֧אִי בַטִּ֛יט וְרִמְסִ֥י בַחֹ֖מֶר הַחֲזִ֥יקִי מַלְבֵּֽן׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 15,
    };
    tanach.add_verse(
        verse_ref,
        "שָׁ֚ם תֹּאכְלֵ֣ךְ אֵ֔שׁ תַּכְרִיתֵ֣ךְ חֶ֔רֶב תֹּאכְלֵ֖ךְ כַּיָּ֑לֶק הִתְכַּבֵּ֣ד כַּיֶּ֔לֶק הִֽתְכַּבְּדִ֖י כָּאַרְבֶּֽה׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 16,
    };
    tanach.add_verse(
        verse_ref,
        "הִרְבֵּית֙ רֹֽכְלַ֔יִךְ מִכּוֹכְבֵ֖י הַשָּׁמָ֑יִם יֶ֥לֶק פָּשַׁ֖ט וַיָּעֹֽף׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 17,
    };
    tanach.add_verse(
        verse_ref,
        "מִנְּזָרַ֙יִךְ֙ כָּֽאַרְבֶּ֔ה וְטַפְסְרַ֖יִךְ כְּג֣וֹב גֹּבָ֑י הַֽחוֹנִ֤ים בַּגְּדֵרוֹת֙ בְּי֣וֹם קָרָ֔ה שֶׁ֤מֶשׁ זָֽרְחָה֙ וְנוֹדַ֔ד וְלֹֽא־נוֹדַ֥ע מְקוֹמ֖וֹ אַיָּֽם׃"
            .to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 18,
    };
    tanach.add_verse(
        verse_ref,
        "נָמ֤וּ רֹעֶ֙יךָ֙ מֶ֣לֶךְ אַשּׁ֔וּר יִשְׁכְּנ֖וּ אַדִּירֶ֑יךָ נָפֹ֧שׁוּ עַמְּךָ֛ עַל־הֶהָרִ֖ים וְאֵ֥ין מְקַבֵּֽץ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Nahum".to_string(),
        chapter_no: 3,
        verse_no: 19,
    };
    tanach.add_verse(
        verse_ref,
        "אֵין־כֵּהָ֣ה לְשִׁבְרֶ֔ךָ נַחְלָ֖ה מַכָּתֶ֑ךָ כֹּ֣ל ׀ שֹׁמְעֵ֣י שִׁמְעֲךָ֗ תָּ֤קְעוּ כַף֙ עָלֶ֔יךָ כִּ֗י עַל־מִ֛י לֹֽא־עָבְרָ֥ה רָעָתְךָ֖ תָּמִֽיד׃"
            .to_string(),
    );
    let book_meta_data = BookMetaData {
        uxlc_version: "1.9".to_string(),
        build_date: "4 Jul 2023  00:00".to_string(),
        build_version: "27.0".to_string(),
        nr_of_chapters: 3,
        nr_of_verses: 56,
        verses_per_chapter: HashMap::new(),
    };
    tanach.add_book("Habakkuk", &book_meta_data.clone());
    tanach.add_verses_per_chapter("Habakkuk".to_string(), 1, 17);

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 1,
        verse_no: 1,
    };
    tanach.add_verse(verse_ref, "הַמַּשָׂא֙ אֲשֶׁ֣ר חָזָ֔ה חֲבַקּ֖וּק הַנָּבִֽיא׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 1,
        verse_no: 2,
    };
    tanach.add_verse(
        verse_ref,
        "עַד־אָ֧נָה יְהוָ֛ה שִׁוַּ֖עְתִּי וְלֹ֣א תִשְׁמָ֑ע אֶזְעַ֥ק אֵלֶ֛יךָ חָמָ֖ס וְלֹ֥א תוֹשִֽׁיעַ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 1,
        verse_no: 3,
    };
    tanach.add_verse(
        verse_ref,
        "לָ֣מָּה תַרְאֵ֤נִי אָ֙וֶן֙ וְעָמָ֣ל תַּבִּ֔יט וְשֹׁ֥ד וְחָמָ֖ס לְנֶגְדִּ֑י וַיְהִ֧י רִ֦יב וּמָד֖וֹן יִשָּֽׂא׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 1,
        verse_no: 4,
    };
    tanach.add_verse(
        verse_ref,
        "עַל־כֵּן֙ תָּפ֣וּג תּוֹרָ֔ה וְלֹֽא־יֵצֵ֥א לָנֶ֖צַח מִשְׁפָּ֑ט כִּ֤י רָשָׁע֙ מַכְתִּ֣יר אֶת־הַצַּדִּ֔יק עַל־כֵּ֛ן יֵצֵ֥א מִשְׁפָּ֖ט מְעֻקָּֽל׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 1,
        verse_no: 5,
    };
    tanach.add_verse(
        verse_ref,
        "רְא֤וּ בַגּוֹיִם֙ וְֽהַבִּ֔יטוּ וְהִֽתַּמְּה֖וּ תְּמָ֑הוּ כִּי־פֹ֙עַל֙ פֹּעֵ֣ל בִּֽימֵיכֶ֔ם לֹ֥א תַאֲמִ֖ינוּ כִּ֥י יְסֻפָּֽר׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 1,
        verse_no: 6,
    };
    tanach.add_verse(
        verse_ref,
        "כִּֽי־הִנְנִ֤י מֵקִים֙ אֶת־הַכַּשְׂדִּ֔ים הַגּ֖וֹי הַמַּ֣ר וְהַנִּמְהָ֑ר הַֽהוֹלֵךְ֙ לְמֶרְחֲבֵי־אֶ֔רֶץ לָרֶ֖שֶׁת מִשְׁכָּנ֥וֹת לֹּא־לֽוֹ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 1,
        verse_no: 7,
    };
    tanach.add_verse(verse_ref, "אָיֹ֥ם וְנוֹרָ֖א ה֑וּא מִמֶּ֕נּוּ מִשְׁפָּט֥וֹ וּשְׂאֵת֖וֹ יֵצֵֽא׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 1,
        verse_no: 8,
    };
    tanach.add_verse(
        verse_ref,
        "וְקַלּ֨וּ מִנְּמֵרִ֜ים סוּסָ֗יו וְחַדּוּ֙ מִזְּאֵ֣בֵי עֶ֔רֶב וּפָ֖שׁוּ פָּֽרָשָׁ֑יו וּפָֽרָשָׁיו֙ מֵרָח֣וֹק יָבֹ֔אוּ יָעֻ֕פוּ כְּנֶ֖שֶׁר חָ֥שׁ לֶאֱכֽוֹל׃"
            .to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 1,
        verse_no: 9,
    };
    tanach.add_verse(
        verse_ref,
        "כֻּלֹּה֙ לְחָמָ֣ס יָב֔וֹא מְגַמַּ֥ת פְּנֵיהֶ֖ם קָדִ֑ימָה וַיֶּאֱסֹ֥ף כַּח֖וֹל שֶֽׁבִי׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 1,
        verse_no: 10,
    };
    tanach.add_verse(
        verse_ref,
        "וְהוּא֙ בַּמְּלָכִ֣ים יִתְקַלָּ֔ס וְרֹזְנִ֖ים מִשְׂחָ֣ק ל֑וֹ ה֚וּא לְכָל־מִבְצָ֣ר יִשְׂחָ֔ק וַיִּצְבֹּ֥ר עָפָ֖ר וַֽיִּלְכְּדָֽהּ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 1,
        verse_no: 11,
    };
    tanach.add_verse(verse_ref, "אָ֣ז חָלַ֥ף ר֛וּחַ וַֽיַּעֲבֹ֖ר וְאָשֵׁ֑ם ז֥וּ כֹח֖וֹ לֵאלֹהֽוֹ׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 1,
        verse_no: 12,
    };
    tanach.add_verse(
        verse_ref,
        "הֲל֧וֹא אַתָּ֣ה מִקֶּ֗דֶם יְהוָ֧ה אֱלֹהַ֛י קְדֹשִׁ֖י לֹ֣א נָמ֑וּת יְהוָה֙ לְמִשְׁפָּ֣ט שַׂמְתּ֔וֹ וְצ֖וּר לְהוֹכִ֥יחַ יְסַדְתּֽוֹ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 1,
        verse_no: 13,
    };
    tanach.add_verse(
        verse_ref,
        "טְה֤וֹר עֵינַ֙יִם֙ מֵרְא֣וֹת רָ֔ע וְהַבִּ֥יט אֶל־עָמָ֖ל לֹ֣א תוּכָ֑ל לָ֤מָּה תַבִּיט֙ בּֽוֹגְדִ֔ים תַּחֲרִ֕ישׁ בְּבַלַּ֥ע רָשָׁ֖ע צַדִּ֥יק מִמֶּֽנּוּ׃"
            .to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 1,
        verse_no: 14,
    };
    tanach.add_verse(verse_ref, "וַתַּעֲשֶׂ֥ה אָדָ֖ם כִּדְגֵ֣י הַיָּ֑ם כְּרֶ֖מֶשׂ לֹא־מֹשֵׁ֥ל בּֽוֹ׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 1,
        verse_no: 15,
    };
    tanach.add_verse(
        verse_ref,
        "כֻּלֹּה֙ בְּחַכָּ֣ה הֵֽעֲלָ֔ה יְגֹרֵ֣הוּ בְחֶרְמ֔וֹ וְיַאַסְפֵ֖הוּ בְּמִכְמַרְתּ֑וֹ עַל־כֵּ֖ן יִשְׂמַ֥ח וְיָגִֽיל׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 1,
        verse_no: 16,
    };
    tanach.add_verse(
        verse_ref,
        "עַל־כֵּן֙ יְזַבֵּ֣חַ לְחֶרְמ֔וֹ וִֽיקַטֵּ֖ר לְמִכְמַרְתּ֑וֹ כִּ֤י בָהֵ֙מָּה֙ שָׁמֵ֣ן חֶלְק֔וֹ וּמַאֲכָל֖וֹ בְּרִאָֽה׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 1,
        verse_no: 17,
    };
    tanach.add_verse(
        verse_ref,
        "הַ֥עַל כֵּ֖ן יָרִ֣יק חֶרְמ֑וֹ וְתָמִ֛יד לַהֲרֹ֥ג גּוֹיִ֖ם לֹ֥א יַחְמֽוֹל׃".to_string(),
    );
    tanach.add_verses_per_chapter("Habakkuk".to_string(), 2, 20);

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 1,
    };
    tanach.add_verse(
        verse_ref,
        "עַל־מִשְׁמַרְתִּ֣י אֶעֱמֹ֔דָה וְאֶֽתְיַצְּבָ֖ה עַל־מָצ֑וֹר וַאֲצַפֶּ֗ה לִרְאוֹת֙ מַה־יְדַבֶּר־בִּ֔י וּמָ֥ה אָשִׁ֖יב עַל־תּוֹכַחְתִּֽי׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 2,
    };
    tanach.add_verse(
        verse_ref,
        "וַיַּעֲנֵ֤נִי יְהוָה֙ וַיֹּ֔אמֶר כְּת֣וֹב חָז֔וֹן וּבָאֵ֖ר עַל־הַלֻּח֑וֹת לְמַ֥עַן יָר֖וּץ ק֥וֹרֵא בֽוֹ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 3,
    };
    tanach.add_verse(
        verse_ref,
        "כִּ֣י ע֤וֹד חָזוֹן֙ לַמּוֹעֵ֔ד וְיָפֵ֥חַ לַקֵּ֖ץ וְלֹ֣א יְכַזֵּ֑ב אִם־יִתְמַהְמָהּ֙ חַכֵּה־ל֔וֹ כִּֽי־בֹ֥א יָבֹ֖א לֹ֥א יְאַחֵֽר׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 4,
    };
    tanach.add_verse(
        verse_ref,
        "הִנֵּ֣ה עֻפְּלָ֔ה לֹא־יָשְׁרָ֥ה נַפְשׁ֖וֹ בּ֑וֹ וְצַדִּ֖יק בֶּאֱמוּנָת֥וֹ יִחְיֶֽה׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 5,
    };
    tanach.add_verse(verse_ref, "וְאַף֙ כִּֽי־הַיַּ֣יִן בּוֹגֵ֔ד גֶּ֥בֶר יָהִ֖יר וְלֹ֣א יִנְוֶ֑ה אֲשֶׁר֩ הִרְחִ֨יב כִּשְׁא֜וֹל נַפְשׁ֗וֹ וְה֤וּא כַמָּ֙וֶת֙ וְלֹ֣א יִשְׂבָּ֔ע וַיֶּאֱסֹ֤ף אֵלָיו֙ כָּל־הַגּוֹיִ֔ם וַיִּקְבֹּ֥ץ אֵלָ֖יו כָּל־הָעַמִּֽים׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 6,
    };
    tanach.add_verse(verse_ref, "הֲלוֹא־אֵ֣לֶּה כֻלָּ֗ם עָלָיו֙ מָשָׁ֣ל יִשָּׂ֔אוּ וּמְלִיצָ֖ה חִיד֣וֹת ל֑וֹ וְיֹאמַ֗ר ה֚וֹי הַמַּרְבֶּ֣ה לֹּא־ל֔וֹ עַד־מָתַ֕י וּמַכְבִּ֥יד עָלָ֖יו עַבְטִֽיט׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 7,
    };
    tanach.add_verse(
        verse_ref,
        "הֲל֣וֹא פֶ֗תַע יָק֙וּמוּ֙ נֹשְׁכֶ֔יךָ וְיִקְצ֖וּ מְזַעְזְעֶ֑יךָ וְהָיִ֥יתָ לִמְשִׁסּ֖וֹת לָֽמוֹ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 8,
    };
    tanach.add_verse(
        verse_ref,
        "כִּֽי אַתָּ֤ה שַׁלּ֙וֹתָ֙ גּוֹיִ֣ם רַבִּ֔ים יְשָׁלּ֖וּךָ כָּל־יֶ֣תֶר עַמִּ֑ים מִדְּמֵ֤י אָדָם֙ וַחֲמַס־אֶ֔רֶץ קִרְיָ֖ה וְכָל־יֹ֥שְׁבֵי בָֽהּ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 9,
    };
    tanach.add_verse(
        verse_ref,
        "ה֗וֹי בֹּצֵ֛עַ בֶּ֥צַע רָ֖ע לְבֵית֑וֹ לָשׂ֤וּם בַּמָּרוֹם֙ קִנּ֔וֹ לְהִנָּצֵ֖ל מִכַּף־רָֽע׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 10,
    };
    tanach.add_verse(
        verse_ref,
        "יָעַ֥צְתָּ בֹּ֖שֶׁת לְבֵיתֶ֑ךָ קְצוֹת־עַמִּ֥ים רַבִּ֖ים וְחוֹטֵ֥א נַפְשֶֽׁךָ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 11,
    };
    tanach.add_verse(verse_ref, "כִּי־אֶ֖בֶן מִקִּ֣יר תִּזְעָ֑ק וְכָפִ֖יס מֵעֵ֥ץ יַעֲנֶֽנָּה׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 12,
    };
    tanach.add_verse(verse_ref, "ה֛וֹי בֹּנֶ֥ה עִ֖יר בְּדָמִ֑ים וְכוֹנֵ֥ן קִרְיָ֖ה בְּעַוְלָֽה׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 13,
    };
    tanach.add_verse(
        verse_ref,
        "הֲל֣וֹא הִנֵּ֔ה מֵאֵ֖ת יְהוָ֣ה צְבָא֑וֹת וְיִֽיגְע֤וּ עַמִּים֙ בְּדֵי־אֵ֔שׁ וּלְאֻמִּ֖ים בְּדֵי־רִ֥יק יִעָֽפוּ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 14,
    };
    tanach.add_verse(
        verse_ref,
        "כִּ֚י תִּמָּלֵ֣א הָאָ֔רֶץ לָדַ֖עַת אֶת־כְּב֣וֹד יְהוָ֑ה כַּמַּ֖יִם יְכַסּ֥וּ עַל־יָֽם׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 15,
    };
    tanach.add_verse(
        verse_ref,
        "ה֚וֹי מַשְׁקֵ֣ה רֵעֵ֔הוּ מְסַפֵּ֥חַ חֲמָתְךָ֖ וְאַ֣ף שַׁכֵּ֑ר לְמַ֥עַן הַבִּ֖יט עַל־מְעוֹרֵיהֶֽם׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 16,
    };
    tanach.add_verse(
        verse_ref,
        "שָׂבַ֤עְתָּ קָלוֹן֙ מִכָּב֔וֹד שְׁתֵ֥ה גַם־אַ֖תָּה וְהֵֽעָרֵ֑ל תִּסּ֣וֹב עָלֶ֗יךָ כּ֚וֹס יְמִ֣ין יְהוָ֔ה וְקִיקָל֖וֹן עַל־כְּבוֹדֶֽךָ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 17,
    };
    tanach.add_verse(
        verse_ref,
        "כִּ֣י חֲמַ֤ס לְבָנוֹן֙ יְכַסֶּ֔ךָּ וְשֹׁ֥ד בְּהֵמ֖וֹת יְחִיתַ֑ן מִדְּמֵ֤י אָדָם֙ וַחֲמַס־אֶ֔רֶץ קִרְיָ֖ה וְכָל־יֹ֥שְׁבֵי בָֽהּ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 18,
    };
    tanach.add_verse(
        verse_ref,
        "מָֽה־הוֹעִ֣יל פֶּ֗סֶל כִּ֤י פְסָלוֹ֙ יֹֽצְר֔וֹ מַסֵּכָ֖ה וּמ֣וֹרֶה שָּׁ֑קֶר כִּ֣י בָטַ֞ח יֹצֵ֤ר יִצְרוֹ֙ עָלָ֔יו לַעֲשׂ֖וֹת אֱלִילִ֥ים אִלְּמִֽים׃"
            .to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 19,
    };
    tanach.add_verse(
        verse_ref,
        "ה֣וֹי אֹמֵ֤ר לָעֵץ֙ הָקִ֔יצָה ע֖וּרִי לְאֶ֣בֶן דּוּמָ֑ם ה֣וּא יוֹרֶ֔ה הִנֵּה־ה֗וּא תָּפוּשׂ֙ זָהָ֣ב וָכֶ֔סֶף וְכָל־ר֖וּחַ אֵ֥ין בְּקִרְבּֽוֹ׃"
            .to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 2,
        verse_no: 20,
    };
    tanach.add_verse(verse_ref, "וַֽיהוָ֖ה בְּהֵיכַ֣ל קָדְשׁ֑וֹ הַ֥ס מִפָּנָ֖יו כָּל־הָאָֽרֶץ׃".to_string());
    tanach.add_verses_per_chapter("Habakkuk".to_string(), 3, 19);

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 1,
    };
    tanach.add_verse(verse_ref, "תְּפִלָּ֖ה לַחֲבַקּ֣וּק הַנָּבִ֑יא עַ֖ל שִׁגְיֹנֽוֹת׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 2,
    };
    tanach.add_verse(
        verse_ref,
        "יְהוָ֗ה שָׁמַ֣עְתִּי שִׁמְעֲךָ֮ יָרֵאתִי֒ יְהוָ֗ה פָּֽעָלְךָ֙ בְּקֶ֤רֶב שָׁנִים֙ חַיֵּ֔יהוּ בְּקֶ֥רֶב שָׁנִ֖ים תּוֹדִ֑יעַ בְּרֹ֖גֶז רַחֵ֥ם תִּזְכּֽוֹר׃"
            .to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 3,
    };
    tanach.add_verse(
        verse_ref,
        "אֱל֙וֹהַ֙ מִתֵּימָ֣ן יָב֔וֹא וְקָד֥וֹשׁ מֵֽהַר־פָּארָ֖ן סֶ֑לָה כִּסָּ֤ה שָׁמַ֙יִם֙ הוֹד֔וֹ וּתְהִלָּת֖וֹ מָלְאָ֥ה הָאָֽרֶץ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 4,
    };
    tanach.add_verse(
        verse_ref,
        "וְנֹ֙גַהּ֙ כָּא֣וֹר תִּֽהְיֶ֔ה קַרְנַ֥יִם מִיָּד֖וֹ ל֑וֹ וְשָׁ֖ם חֶבְי֥וֹן עֻזֹּֽה׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 5,
    };
    tanach.add_verse(verse_ref, "לְפָנָ֖יו יֵ֣לֶךְ דָּ֑בֶר וְיֵצֵ֥א רֶ֖שֶׁף לְרַגְלָֽיו׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 6,
    };
    tanach.add_verse(
        verse_ref,
        "עָמַ֣ד ׀ וַיְמֹ֣דֶד אֶ֗רֶץ רָאָה֙ וַיַּתֵּ֣ר גּוֹיִ֔ם וַיִּתְפֹּֽצְצוּ֙ הַרְרֵי־עַ֔ד שַׁח֖וּ גִּבְע֣וֹת עוֹלָ֑ם הֲלִיכ֥וֹת עוֹלָ֖ם לֽוֹ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 7,
    };
    tanach.add_verse(
        verse_ref,
        "תַּ֣חַת אָ֔וֶן רָאִ֖יתִי אָהֳלֵ֣י כוּשָׁ֑ן יִרְגְּז֕וּן יְרִיע֖וֹת אֶ֥רֶץ מִדְיָֽן׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 8,
    };
    tanach.add_verse(
        verse_ref,
        "הֲבִנְהָרִים֙ חָרָ֣ה יְהוָ֔ה אִ֤ם בַּנְּהָרִים֙ אַפֶּ֔ךָ אִם־בַּיָּ֖ם עֶבְרָתֶ֑ךָ כִּ֤י תִרְכַּב֙ עַל־סוּסֶ֔יךָ מַרְכְּבֹתֶ֖יךָ יְשׁוּעָֽה׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 9,
    };
    tanach.add_verse(
        verse_ref,
        "עֶרְיָ֤ה תֵעוֹר֙ קַשְׁתֶּ֔ךָ שְׁבֻע֥וֹת מַטּ֖וֹת אֹ֣מֶר סֶ֑לָה נְהָר֖וֹת תְּבַקַּע־אָֽרֶץ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 10,
    };
    tanach.add_verse(
        verse_ref,
        "רָא֤וּךָ יָחִ֙ילוּ֙ הָרִ֔ים זֶ֥רֶם מַ֖יִם עָבָ֑ר נָתַ֤ן תְּהוֹם֙ קוֹל֔וֹ ר֖וֹם יָדֵ֥יהוּ נָשָֽׂא׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 11,
    };
    tanach.add_verse(
        verse_ref,
        "שֶׁ֥מֶשׁ יָרֵ֖חַ עָ֣מַד זְבֻ֑לָה לְא֤וֹר חִצֶּ֙יךָ֙ יְהַלֵּ֔כוּ לְנֹ֖גַהּ בְּרַ֥ק חֲנִיתֶֽךָ׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 12,
    };
    tanach.add_verse(verse_ref, "בְּזַ֖עַם תִּצְעַד־אָ֑רֶץ בְּאַ֖ף תָּד֥וּשׁ גּוֹיִֽם׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 13,
    };
    tanach.add_verse(
        verse_ref,
        "יָצָ֙אתָ֙ לְיֵ֣שַׁע עַמֶּ֔ךָ לְיֵ֖שַׁע אֶת־מְשִׁיחֶ֑ךָ מָחַ֤צְתָּ רֹּאשׁ֙ מִבֵּ֣ית רָשָׁ֔ע עָר֛וֹת יְס֥וֹד עַד־צַוָּ֖אר סֶֽלָה׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 14,
    };
    tanach.add_verse(
        verse_ref,
        "נָקַ֤בְתָּ בְמַטָּיו֙ רֹ֣אשׁ *פרזו **פְּרָזָ֔יו יִסְעֲר֖וּ לַהֲפִיצֵ֑נִי עֲלִ֣יצֻתָ֔ם כְּמוֹ־לֶאֱכֹ֥ל עָנִ֖י בַּמִּסְתָּֽר׃".to_string(),
    );

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 15,
    };
    tanach.add_verse(verse_ref, "דָּרַ֥כְתָּ בַיָּ֖ם סוּסֶ֑יךָ חֹ֖מֶר מַ֥יִם רַבִּֽים׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 16,
    };
    tanach.add_verse(verse_ref, "שָׁמַ֣עְתִּי ׀ וַתִּרְגַּ֣ז בִּטְנִ֗י לְקוֹל֙ צָלֲל֣וּ שְׂפָתַ֔י יָב֥וֹא רָקָ֛ב בַּעֲצָמַ֖י וְתַחְתַּ֣י אֶרְגָּ֑ז אֲשֶׁ֤ר אָנ֙וּחַ֙ לְי֣וֹם צָרָ֔ה לַעֲל֖וֹת לְעַ֥ם יְגוּדֶֽנּוּ׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 17,
    };
    tanach.add_verse(verse_ref, "כִּֽי־תְאֵנָ֣ה לֹֽא־תִפְרָ֗ח וְאֵ֤ין יְבוּל֙ בַּגְּפָנִ֔ים כִּחֵשׁ֙ מַעֲשֵׂה־זַ֔יִת וּשְׁדֵמ֖וֹת לֹא־עָ֣שָׂה אֹ֑כֶל גָּזַ֤ר מִמִּכְלָה֙ צֹ֔אן וְאֵ֥ין בָּקָ֖ר בָּרְפָתִֽים׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 18,
    };
    tanach.add_verse(verse_ref, "וַאֲנִ֖י בַּיהוָ֣ה אֶעְל֑וֹזָה אָגִ֖ילָה בֵּאלֹהֵ֥י יִשְׁעִֽי׃".to_string());

    let verse_ref = VerseReference {
        book_name: "Habakkuk".to_string(),
        chapter_no: 3,
        verse_no: 19,
    };
    tanach.add_verse(
        verse_ref,
        "יְהוִ֤הּ אֲדֹנָי֙ חֵילִ֔י וַיָּ֤שֶׂם רַגְלַי֙ כָּֽאַיָּל֔וֹת וְעַ֥ל בָּמוֹתַ֖י יַדְרִכֵ֑נִי לַמְנַצֵּ֖חַ בִּנְגִינוֹתָֽי׃".to_string(),
    );
    tanach
}
// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//                     ===> END OF GENERATED CODE <===                  +
// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

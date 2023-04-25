use crate::db::connection;

#[allow(dead_code)]
pub fn populate() {
    let connection = match connection::connect() {
        Ok(conn) => conn,
        Err(err) => {
            println!("{:?}", err.message);
            panic!("Failed to connect to the database");
        }
    };

    let query = "
        CREATE TABLE IF NOT EXISTS verses (
          id INTEGER PRIMARY KEY, transliteration TEXT, 
          translation TEXT, chapter INTEGER, 
          position INTEGER
        );
        insert into verses 
        values 
          (
            1, 'dharma-kṣhetre kuru-kṣhetre samavetā yuyutsavaḥ 
        māmakāḥ pāṇḍavāśhchaiva kimakurvata sañjaya', 
            'dhritarastra said: o sanjaya, what did my sons and the sons of pandu actually do when, eager for battle, they gathered together on the holy field of kuruksetra?', 
            1, 1
          );
        insert into verses 
        values 
          (
            2, 'dṛiṣhṭvā tu pāṇḍavānīkaṁ vyūḍhaṁ duryodhanastadā 
        āchāryamupasaṅgamya rājā vachanamabravīt', 
            'Sanjaya said: But then, seeing the army of the Pandavas in battle array, King Duryodhana approached his teacher (Dronacharya) and spoke the following words:', 
            1, 2
          );
        insert into verses 
        values 
          (
            3, 'paśhyaitāṁ pāṇḍu-putrāṇām āchārya mahatīṁ chamūm 
        vyūḍhāṁ drupada-putreṇa tava śhiṣhyeṇa dhīmatā', 
            'O teacher, (please) see this vast army of the sons of Pandu, arrayed for battle by the son of Drupada, your intelligent disciple, Dhrishtadyumna.', 
            1, 3
          );
        insert into verses 
        values 
          (
            4, 'atra śhūrā maheṣhvāsā bhīmārjuna-samā yudhi 
        yuyudhāno virāṭaśhcha drupadaśhcha mahā-rathaḥ', 
            'There are in this army, heroes wielding great bows, and equal in military prowess to Bhima and Arjuna: Yuyudhana (Satyaki) and Virata, and the maharatha (great chariot-rider) Drupada;', 
            1, 4
          );
        insert into verses 
        values 
          (
            5, 'dhṛiṣhṭaketuśhchekitānaḥ kāśhirājaśhcha vīryavān
        purujit kuntibhojaśhcha śhaibyaśhcha nara-puṅgavaḥ
        yudhāmanyuśhcha vikrānta uttamaujāśhcha vīryavān', 
            'Dhrstaketu, Cekitana, and the valiant king of Kasi (Varanasi); Purujit and Kuntibhoja, and Saibya, the choicest among men;', 
            1, 5
          );
        insert into verses 
        values 
          (
            6, 'saubhadro draupadeyāśhcha sarva eva mahā-rathāḥ', 
            'And the chivalrous Yudhamanyu, and the valiant Uttamaujas; son of Subhadra (Abhimanyu) and the sons of Draupadi all (of whom) are, verily, maharathas.', 
            1, 6
          );
        insert into verses 
        values 
          (
            7, 'asmākaṁ tu viśhiṣhṭā ye tānnibodha dwijottama
        nāyakā mama sainyasya sanjñārthaṁ tānbravīmi te', 
            'But, O best among the Brahmanas, please be appraised of those who are foremost among us, the principal warriors of my army. I speak of them to you by way of example.', 
            1, 7
          );
        insert into verses 
        values 
          (
            8, 'bhavānbhīṣhmaśhcha karṇaśhcha kṛipaśhcha samitiñjayaḥ
        aśhvatthāmā vikarṇaśhcha saumadattis tathaiva cha', 
            '(They are:) Your venerable self, Bhisma and Karna, and Krpa who is ever victorious in battle; Asvatthama, Vikarna, Saumadatti and Jayadratha.', 
            1, 8
          );
        insert into verses 
        values 
          (
            9, 'anye cha bahavaḥ śhūrā madarthe tyaktajīvitāḥ
        nānā-śhastra-praharaṇāḥ sarve yuddha-viśhāradāḥ', 
            'There are many heroes who have dedicated their lives for my sake, who possess various kinds of weapons and missiles, (and) all of whom are skilled in battle.', 
            1, 9
          );
        insert into verses 
        values 
          (
            10, 'aparyāptaṁ tadasmākaṁ balaṁ bhīṣhmābhirakṣhitam
        paryāptaṁ tvidameteṣhāṁ balaṁ bhīmābhirakṣhitam', 
            'Therefore, our army under the complete protection of Bhisma and others is unlimited. But the army of these (enemies), under the protection of Bhima and others is limited.', 
            1, 10
          );
        ";
    connection.execute(query).unwrap();
}

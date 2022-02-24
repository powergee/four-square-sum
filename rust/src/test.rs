use super::*;

#[test]
fn test_helper_functions() {
    assert_eq!(is_perfect_square(&BigInt::from(1)), true);
    assert_eq!(is_perfect_square(&BigInt::from(2)), false);
    assert_eq!(is_perfect_square(&BigInt::from(9)), true);
    assert_eq!(is_perfect_square(&BigInt::from(10)), false);
    assert_eq!(is_perfect_square(&BigInt::from(239484895234_i64).pow(2)), true);
}

#[test]
fn test_miller_rabin() {
    assert_eq!(is_prime(&BigInt::from(1_i32)), false);
    assert_eq!(is_prime(&BigInt::from(2_i32)), true);
    assert_eq!(is_prime(&BigInt::from(3_i32)), true);
    assert_eq!(is_prime(&BigInt::from(4_i32)), false);
    assert_eq!(is_prime(&BigInt::from(11_i32)), true);
    assert_eq!(is_prime(&BigInt::from(4397_i32)), true);
    assert_eq!(is_prime(&"18446744073709551557".parse::<BigInt>().unwrap()), true);
}

#[test]
fn test_pollard_rho() {
    assert_eq!(validate_factors(&factorize(&BigInt::from(2_i32)), &vec![BigInt::from(2_i32)]), true);
    assert_eq!(validate_factors(&factorize(&BigInt::from(3_i32)), &vec![BigInt::from(3_i32)]), true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(128_i32)),
            &vec![
                BigInt::from(2_i32),
                BigInt::from(2_i32),
                BigInt::from(2_i32),
                BigInt::from(2_i32),
                BigInt::from(2_i32),
                BigInt::from(2_i32),
                BigInt::from(2_i32)
            ]),
        true);
    
    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(1804289383_i64)),
            &vec![
                BigInt::from(13_i64),
                BigInt::from(421_i64),
                BigInt::from(329671_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(424238335_i64)),
            &vec![
                BigInt::from(5_i64),
                BigInt::from(23_i64),
                BigInt::from(157_i64),
                BigInt::from(23497_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(783368690_i64)),
            &vec![
                BigInt::from(2_i64),
                BigInt::from(5_i64),
                BigInt::from(13_i64),
                BigInt::from(67_i64),
                BigInt::from(89939_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(1303455736_i64)),
            &vec![
                BigInt::from(2_i64),
                BigInt::from(2_i64),
                BigInt::from(2_i64),
                BigInt::from(11_i64),
                BigInt::from(97_i64),
                BigInt::from(311_i64),
                BigInt::from(491_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(278722862_i64)),
            &vec![
                BigInt::from(2_i64),
                BigInt::from(11_i64),
                BigInt::from(113_i64),
                BigInt::from(191_i64),
                BigInt::from(587_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(635723058_i64)),
            &vec![
                BigInt::from(2_i64),
                BigInt::from(3_i64),
                BigInt::from(17_i64),
                BigInt::from(823_i64),
                BigInt::from(7573_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(1131176229_i64)),
            &vec![
                BigInt::from(3_i64),
                BigInt::from(19_i64),
                BigInt::from(19845197_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(756898537_i64)),
            &vec![
                BigInt::from(127_i64),
                BigInt::from(5959831_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(149798315_i64)),
            &vec![
                BigInt::from(5_i64),
                BigInt::from(181_i64),
                BigInt::from(165523_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(1424268980_i64)),
            &vec![
                BigInt::from(2_i64),
                BigInt::from(2_i64),
                BigInt::from(5_i64),
                BigInt::from(3331_i64),
                BigInt::from(21379_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(1842800140108386207_i64)),
            &vec![
                BigInt::from(3_i64),
                BigInt::from(3_i64),
                BigInt::from(3_i64),
                BigInt::from(7_i64),
                BigInt::from(7_i64),
                BigInt::from(7_i64),
                BigInt::from(521_i64),
                BigInt::from(881_i64),
                BigInt::from(433517587_i64),
            ]),
        true);
    
    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(3535206131977819650_i64)),
            &vec![
                BigInt::from(2_i64),
                BigInt::from(3_i64),
                BigInt::from(3_i64),
                BigInt::from(3_i64),
                BigInt::from(5_i64),
                BigInt::from(5_i64),
                BigInt::from(13_i64),
                BigInt::from(97_i64),
                BigInt::from(149_i64),
                BigInt::from(12689_i64),
                BigInt::from(1098379_i64),
            ]),
        true);
    
    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(2377334280519188208_i64)),
            &vec![
                BigInt::from(2_i64),
                BigInt::from(2_i64),
                BigInt::from(2_i64),
                BigInt::from(2_i64),
                BigInt::from(3_i64),
                BigInt::from(3_i64),
                BigInt::from(31_i64),
                BigInt::from(4084163_i64),
                BigInt::from(130395619_i64),
            ]),
        true);
    
    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(2343112343973940266_i64)),
            &vec![
                BigInt::from(2_i64),
                BigInt::from(3_i64),
                BigInt::from(3_i64),
                BigInt::from(7_i64),
                BigInt::from(13_i64),
                BigInt::from(31_i64),
                BigInt::from(701_i64),
                BigInt::from(1187_i64),
                BigInt::from(1621_i64),
                BigInt::from(34211_i64),
            ]),
        true);
    
    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(2513304378703685578_i64)),
            &vec![
                BigInt::from(2_i64),
                BigInt::from(11_i64),
                BigInt::from(139724383_i64),
                BigInt::from(817617553_i64),
            ]),
        true);
    
    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(1600684248743118332_i64)),
            &vec![
                BigInt::from(2_i64),
                BigInt::from(2_i64),
                BigInt::from(29_i64),
                BigInt::from(11476537_i64),
                BigInt::from(1202366371_i64),
            ]),
        true);
    
    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(2436683988946862403_i64)),
            &vec![
                BigInt::from(3_i64),
                BigInt::from(3_i64),
                BigInt::from(3_i64),
                BigInt::from(7_i64),
                BigInt::from(7_i64),
                BigInt::from(199_i64),
                BigInt::from(1001501_i64),
                BigInt::from(9241339_i64),
            ]),
        true);
    
    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(2432874024842030420_i64)),
            &vec![
                BigInt::from(2_i64),
                BigInt::from(2_i64),
                BigInt::from(5_i64),
                BigInt::from(7_i64),
                BigInt::from(7_i64),
                BigInt::from(41_i64),
                BigInt::from(571_i64),
                BigInt::from(3271_i64),
                BigInt::from(4517_i64),
                BigInt::from(7177_i64),
            ]),
        true);
    
    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(1720765833189614448_i64)),
            &vec![
                BigInt::from(2_i64),
                BigInt::from(2_i64),
                BigInt::from(2_i64),
                BigInt::from(2_i64),
                BigInt::from(3_i64),
                BigInt::from(19_i64),
                BigInt::from(19_i64),
                BigInt::from(149_i64),
                BigInt::from(1259_i64),
                BigInt::from(2437_i64),
                BigInt::from(217223_i64),
            ]),
        true);
    
    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(2849458758673964367_i64)),
            &vec![
                BigInt::from(3_i64),
                BigInt::from(7_i64),
                BigInt::from(7_i64),
                BigInt::from(7_i64),
                BigInt::from(53_i64),
                BigInt::from(241051_i64),
                BigInt::from(216751541_i64),
            ]),
        true);
}

#[test]
fn test_four_square_random_but_fast() {
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1_i32), false)), BigInt::from(1_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2_i32), false)), BigInt::from(2_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(3_i32), false)), BigInt::from(3_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(123_i32), false)), BigInt::from(123_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(123232_i32), false)), BigInt::from(123232_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1804289383_i64), false)), BigInt::from(1804289383_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(846930886_i64), false)), BigInt::from(846930886_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1681692777_i64), false)), BigInt::from(1681692777_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1714636915_i64), false)), BigInt::from(1714636915_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1957747793_i64), false)), BigInt::from(1957747793_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(424238335_i64), false)), BigInt::from(424238335_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(719885386_i64), false)), BigInt::from(719885386_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1649760492_i64), false)), BigInt::from(1649760492_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(596516649_i64), false)), BigInt::from(596516649_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1189641421_i64), false)), BigInt::from(1189641421_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1842800140108386207_i64), false)), BigInt::from(1842800140108386207_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2319442335322128615_i64), false)), BigInt::from(2319442335322128615_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2177281983706900224_i64), false)), BigInt::from(2177281983706900224_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2243420669815203258_i64), false)), BigInt::from(2243420669815203258_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(3150919813662214902_i64), false)), BigInt::from(3150919813662214902_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2099491376865084255_i64), false)), BigInt::from(2099491376865084255_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2890157740621932327_i64), false)), BigInt::from(2890157740621932327_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1474775625792949608_i64), false)), BigInt::from(1474775625792949608_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(3119718353897036920_i64), false)), BigInt::from(3119718353897036920_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(3535206131977819650_i64), false)), BigInt::from(3535206131977819650_i64));
    assert_eq!(get_square_sum(&find_solution(&"556241405847937766541".parse::<BigInt>().unwrap(), false)), "556241405847937766541".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"8035281690184899849238".parse::<BigInt>().unwrap(), false)), "8035281690184899849238".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"5666108380855761692650".parse::<BigInt>().unwrap(), false)), "5666108380855761692650".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"99253663622552392578064".parse::<BigInt>().unwrap(), false)), "99253663622552392578064".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"7561187792538186301056".parse::<BigInt>().unwrap(), false)), "7561187792538186301056".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"2497474809640410455936".parse::<BigInt>().unwrap(), false)), "2497474809640410455936".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"7105174348442182498825".parse::<BigInt>().unwrap(), false)), "7105174348442182498825".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"31790059461098425261609".parse::<BigInt>().unwrap(), false)), "31790059461098425261609".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"881206347039301784985".parse::<BigInt>().unwrap(), false)), "881206347039301784985".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"50711412752875721635687".parse::<BigInt>().unwrap(), false)), "50711412752875721635687".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"4637682800936075503470".parse::<BigInt>().unwrap(), false)), "4637682800936075503470".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"89235216991850132338155".parse::<BigInt>().unwrap(), false)), "89235216991850132338155".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"45172441386009800985111".parse::<BigInt>().unwrap(), false)), "45172441386009800985111".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"26291774512978227127227".parse::<BigInt>().unwrap(), false)), "26291774512978227127227".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"8883199051190137259141".parse::<BigInt>().unwrap(), false)), "8883199051190137259141".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"227879800883102175568".parse::<BigInt>().unwrap(), false)), "227879800883102175568".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"5507411864235925943004".parse::<BigInt>().unwrap(), false)), "5507411864235925943004".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"3870340167134272034306".parse::<BigInt>().unwrap(), false)), "3870340167134272034306".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"871071798590050117261".parse::<BigInt>().unwrap(), false)), "871071798590050117261".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"956921988759753441131".parse::<BigInt>().unwrap(), false)), "956921988759753441131".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"1341080445124336333152".parse::<BigInt>().unwrap(), false)), "1341080445124336333152".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"192108248502792357502".parse::<BigInt>().unwrap(), false)), "192108248502792357502".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"80495547348559595467658".parse::<BigInt>().unwrap(), false)), "80495547348559595467658".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"9474614118265868246951".parse::<BigInt>().unwrap(), false)), "9474614118265868246951".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"3560328314172234648297".parse::<BigInt>().unwrap(), false)), "3560328314172234648297".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"29569079356508541270415".parse::<BigInt>().unwrap(), false)), "29569079356508541270415".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"4192362377160083540845".parse::<BigInt>().unwrap(), false)), "4192362377160083540845".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"79394444285411641657677".parse::<BigInt>().unwrap(), false)), "79394444285411641657677".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"931378768054620806417".parse::<BigInt>().unwrap(), false)), "931378768054620806417".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"489485176617187077021".parse::<BigInt>().unwrap(), false)), "489485176617187077021".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"6278841778864818596888".parse::<BigInt>().unwrap(), false)), "6278841778864818596888".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"286310607895837827884".parse::<BigInt>().unwrap(), false)), "286310607895837827884".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"24320200169912974683489".parse::<BigInt>().unwrap(), false)), "24320200169912974683489".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"6304462708012013622963".parse::<BigInt>().unwrap(), false)), "6304462708012013622963".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"407665794090485483415".parse::<BigInt>().unwrap(), false)), "407665794090485483415".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"9037424716982561422628".parse::<BigInt>().unwrap(), false)), "9037424716982561422628".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"544086731760106923827".parse::<BigInt>().unwrap(), false)), "544086731760106923827".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"49003633228756321409630".parse::<BigInt>().unwrap(), false)), "49003633228756321409630".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"82654515417902325390122".parse::<BigInt>().unwrap(), false)), "82654515417902325390122".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"5445840288753008279204".parse::<BigInt>().unwrap(), false)), "5445840288753008279204".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"57471816960665605480700".parse::<BigInt>().unwrap(), false)), "57471816960665605480700".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"4076031917645906502378".parse::<BigInt>().unwrap(), false)), "4076031917645906502378".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"7443244176038467733112".parse::<BigInt>().unwrap(), false)), "7443244176038467733112".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"801239092537983079327".parse::<BigInt>().unwrap(), false)), "801239092537983079327".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"3466977407431941988469".parse::<BigInt>().unwrap(), false)), "3466977407431941988469".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"423956647303293892425".parse::<BigInt>().unwrap(), false)), "423956647303293892425".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"36577136638945729659200".parse::<BigInt>().unwrap(), false)), "36577136638945729659200".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"9993469326217605107564".parse::<BigInt>().unwrap(), false)), "9993469326217605107564".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"9159359051105790172570".parse::<BigInt>().unwrap(), false)), "9159359051105790172570".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"8179834617322814327696".parse::<BigInt>().unwrap(), false)), "8179834617322814327696".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"87544261489684267999371".parse::<BigInt>().unwrap(), false)), "87544261489684267999371".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"6549526611263970986738".parse::<BigInt>().unwrap(), false)), "6549526611263970986738".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"69322351807979740968478".parse::<BigInt>().unwrap(), false)), "69322351807979740968478".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"66461224378705807595679".parse::<BigInt>().unwrap(), false)), "66461224378705807595679".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"3774551325950113911161".parse::<BigInt>().unwrap(), false)), "3774551325950113911161".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"5931502489236960111915".parse::<BigInt>().unwrap(), false)), "5931502489236960111915".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"66209013607787948183789".parse::<BigInt>().unwrap(), false)), "66209013607787948183789".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"921068356555769496195".parse::<BigInt>().unwrap(), false)), "921068356555769496195".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"354201011144424770245".parse::<BigInt>().unwrap(), false)), "354201011144424770245".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"465149022676889812178".parse::<BigInt>().unwrap(), false)), "465149022676889812178".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"622846428783052376944".parse::<BigInt>().unwrap(), false)), "622846428783052376944".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"947868574798565083966".parse::<BigInt>().unwrap(), false)), "947868574798565083966".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"81209374038916873445211".parse::<BigInt>().unwrap(), false)), "81209374038916873445211".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"89227141463320744547444".parse::<BigInt>().unwrap(), false)), "89227141463320744547444".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"304245276624767340763".parse::<BigInt>().unwrap(), false)), "304245276624767340763".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"772766335330795194810".parse::<BigInt>().unwrap(), false)), "772766335330795194810".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"8715984247184847102629".parse::<BigInt>().unwrap(), false)), "8715984247184847102629".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"317398389294290667043".parse::<BigInt>().unwrap(), false)), "317398389294290667043".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"4211495454540623438602".parse::<BigInt>().unwrap(), false)), "4211495454540623438602".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"283880381614019685135".parse::<BigInt>().unwrap(), false)), "283880381614019685135".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"39619211139710770832433".parse::<BigInt>().unwrap(), false)), "39619211139710770832433".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"7185665471565876183582".parse::<BigInt>().unwrap(), false)), "7185665471565876183582".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"4073428941008652389939".parse::<BigInt>().unwrap(), false)), "4073428941008652389939".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"126797938203833874094".parse::<BigInt>().unwrap(), false)), "126797938203833874094".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"161472960118007947140".parse::<BigInt>().unwrap(), false)), "161472960118007947140".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"35090086855192311522542".parse::<BigInt>().unwrap(), false)), "35090086855192311522542".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"8807539481503471142267".parse::<BigInt>().unwrap(), false)), "8807539481503471142267".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"347391647676107808333".parse::<BigInt>().unwrap(), false)), "347391647676107808333".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"742685478636929796464".parse::<BigInt>().unwrap(), false)), "742685478636929796464".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"23501936070330009862657".parse::<BigInt>().unwrap(), false)), "23501936070330009862657".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"666741444048104377839".parse::<BigInt>().unwrap(), false)), "666741444048104377839".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"79913760518204578973905".parse::<BigInt>().unwrap(), false)), "79913760518204578973905".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"72527420513192143286633".parse::<BigInt>().unwrap(), false)), "72527420513192143286633".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"30042727171944117526763".parse::<BigInt>().unwrap(), false)), "30042727171944117526763".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"718671323585233404360".parse::<BigInt>().unwrap(), false)), "718671323585233404360".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"68126089383172504864607".parse::<BigInt>().unwrap(), false)), "68126089383172504864607".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"3059866869120125278993".parse::<BigInt>().unwrap(), false)), "3059866869120125278993".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"62710819416614400313547".parse::<BigInt>().unwrap(), false)), "62710819416614400313547".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"96955472843824997659299".parse::<BigInt>().unwrap(), false)), "96955472843824997659299".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"2282652533088726475824".parse::<BigInt>().unwrap(), false)), "2282652533088726475824".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"724255300584456982182".parse::<BigInt>().unwrap(), false)), "724255300584456982182".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"70597314356773333583940".parse::<BigInt>().unwrap(), false)), "70597314356773333583940".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"1817830065030373843878".parse::<BigInt>().unwrap(), false)), "1817830065030373843878".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"35166875377282288902041".parse::<BigInt>().unwrap(), false)), "35166875377282288902041".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"458456703989975254758".parse::<BigInt>().unwrap(), false)), "458456703989975254758".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"87161122660537307917805".parse::<BigInt>().unwrap(), false)), "87161122660537307917805".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"52947832740072441614366".parse::<BigInt>().unwrap(), false)), "52947832740072441614366".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"1826071513983522132178".parse::<BigInt>().unwrap(), false)), "1826071513983522132178".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"858406154970963776871".parse::<BigInt>().unwrap(), false)), "858406154970963776871".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"23747828523575180599785".parse::<BigInt>().unwrap(), false)), "23747828523575180599785".parse::<BigInt>().unwrap());
}

#[test]
fn test_four_square_optimal_but_slow() {
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1_i32), true)), BigInt::from(1_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2_i32), true)), BigInt::from(2_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(3_i32), true)), BigInt::from(3_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(123_i32), true)), BigInt::from(123_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(123232_i32), true)), BigInt::from(123232_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1804289383_i64), true)), BigInt::from(1804289383_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(846930886_i64), true)), BigInt::from(846930886_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1681692777_i64), true)), BigInt::from(1681692777_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1714636915_i64), true)), BigInt::from(1714636915_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1957747793_i64), true)), BigInt::from(1957747793_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(424238335_i64), true)), BigInt::from(424238335_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(719885386_i64), true)), BigInt::from(719885386_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1649760492_i64), true)), BigInt::from(1649760492_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(596516649_i64), true)), BigInt::from(596516649_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1189641421_i64), true)), BigInt::from(1189641421_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1842800140108386207_i64), true)), BigInt::from(1842800140108386207_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2319442335322128615_i64), true)), BigInt::from(2319442335322128615_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2177281983706900224_i64), true)), BigInt::from(2177281983706900224_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2243420669815203258_i64), true)), BigInt::from(2243420669815203258_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(3150919813662214902_i64), true)), BigInt::from(3150919813662214902_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2099491376865084255_i64), true)), BigInt::from(2099491376865084255_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2890157740621932327_i64), true)), BigInt::from(2890157740621932327_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1474775625792949608_i64), true)), BigInt::from(1474775625792949608_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(3119718353897036920_i64), true)), BigInt::from(3119718353897036920_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(3535206131977819650_i64), true)), BigInt::from(3535206131977819650_i64));
    assert_eq!(get_square_sum(&find_solution(&"556241405847937766541".parse::<BigInt>().unwrap(), true)), "556241405847937766541".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"8035281690184899849238".parse::<BigInt>().unwrap(), true)), "8035281690184899849238".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"5666108380855761692650".parse::<BigInt>().unwrap(), true)), "5666108380855761692650".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"99253663622552392578064".parse::<BigInt>().unwrap(), true)), "99253663622552392578064".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"7561187792538186301056".parse::<BigInt>().unwrap(), true)), "7561187792538186301056".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"2497474809640410455936".parse::<BigInt>().unwrap(), true)), "2497474809640410455936".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"7105174348442182498825".parse::<BigInt>().unwrap(), true)), "7105174348442182498825".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"31790059461098425261609".parse::<BigInt>().unwrap(), true)), "31790059461098425261609".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"881206347039301784985".parse::<BigInt>().unwrap(), true)), "881206347039301784985".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"50711412752875721635687".parse::<BigInt>().unwrap(), true)), "50711412752875721635687".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"4637682800936075503470".parse::<BigInt>().unwrap(), true)), "4637682800936075503470".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"89235216991850132338155".parse::<BigInt>().unwrap(), true)), "89235216991850132338155".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"45172441386009800985111".parse::<BigInt>().unwrap(), true)), "45172441386009800985111".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"26291774512978227127227".parse::<BigInt>().unwrap(), true)), "26291774512978227127227".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"8883199051190137259141".parse::<BigInt>().unwrap(), true)), "8883199051190137259141".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"227879800883102175568".parse::<BigInt>().unwrap(), true)), "227879800883102175568".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"5507411864235925943004".parse::<BigInt>().unwrap(), true)), "5507411864235925943004".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"3870340167134272034306".parse::<BigInt>().unwrap(), true)), "3870340167134272034306".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"871071798590050117261".parse::<BigInt>().unwrap(), true)), "871071798590050117261".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"956921988759753441131".parse::<BigInt>().unwrap(), true)), "956921988759753441131".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"1341080445124336333152".parse::<BigInt>().unwrap(), true)), "1341080445124336333152".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"192108248502792357502".parse::<BigInt>().unwrap(), true)), "192108248502792357502".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"80495547348559595467658".parse::<BigInt>().unwrap(), true)), "80495547348559595467658".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"9474614118265868246951".parse::<BigInt>().unwrap(), true)), "9474614118265868246951".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"3560328314172234648297".parse::<BigInt>().unwrap(), true)), "3560328314172234648297".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"29569079356508541270415".parse::<BigInt>().unwrap(), true)), "29569079356508541270415".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"4192362377160083540845".parse::<BigInt>().unwrap(), true)), "4192362377160083540845".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"79394444285411641657677".parse::<BigInt>().unwrap(), true)), "79394444285411641657677".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"931378768054620806417".parse::<BigInt>().unwrap(), true)), "931378768054620806417".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"489485176617187077021".parse::<BigInt>().unwrap(), true)), "489485176617187077021".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"6278841778864818596888".parse::<BigInt>().unwrap(), true)), "6278841778864818596888".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"286310607895837827884".parse::<BigInt>().unwrap(), true)), "286310607895837827884".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"24320200169912974683489".parse::<BigInt>().unwrap(), true)), "24320200169912974683489".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"6304462708012013622963".parse::<BigInt>().unwrap(), true)), "6304462708012013622963".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"407665794090485483415".parse::<BigInt>().unwrap(), true)), "407665794090485483415".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"9037424716982561422628".parse::<BigInt>().unwrap(), true)), "9037424716982561422628".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"544086731760106923827".parse::<BigInt>().unwrap(), true)), "544086731760106923827".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"49003633228756321409630".parse::<BigInt>().unwrap(), true)), "49003633228756321409630".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"82654515417902325390122".parse::<BigInt>().unwrap(), true)), "82654515417902325390122".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"5445840288753008279204".parse::<BigInt>().unwrap(), true)), "5445840288753008279204".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"57471816960665605480700".parse::<BigInt>().unwrap(), true)), "57471816960665605480700".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"4076031917645906502378".parse::<BigInt>().unwrap(), true)), "4076031917645906502378".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"7443244176038467733112".parse::<BigInt>().unwrap(), true)), "7443244176038467733112".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"801239092537983079327".parse::<BigInt>().unwrap(), true)), "801239092537983079327".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"3466977407431941988469".parse::<BigInt>().unwrap(), true)), "3466977407431941988469".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"423956647303293892425".parse::<BigInt>().unwrap(), true)), "423956647303293892425".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"36577136638945729659200".parse::<BigInt>().unwrap(), true)), "36577136638945729659200".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"9993469326217605107564".parse::<BigInt>().unwrap(), true)), "9993469326217605107564".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"9159359051105790172570".parse::<BigInt>().unwrap(), true)), "9159359051105790172570".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"8179834617322814327696".parse::<BigInt>().unwrap(), true)), "8179834617322814327696".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"87544261489684267999371".parse::<BigInt>().unwrap(), true)), "87544261489684267999371".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"6549526611263970986738".parse::<BigInt>().unwrap(), true)), "6549526611263970986738".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"69322351807979740968478".parse::<BigInt>().unwrap(), true)), "69322351807979740968478".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"66461224378705807595679".parse::<BigInt>().unwrap(), true)), "66461224378705807595679".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"3774551325950113911161".parse::<BigInt>().unwrap(), true)), "3774551325950113911161".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"5931502489236960111915".parse::<BigInt>().unwrap(), true)), "5931502489236960111915".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"66209013607787948183789".parse::<BigInt>().unwrap(), true)), "66209013607787948183789".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"921068356555769496195".parse::<BigInt>().unwrap(), true)), "921068356555769496195".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"354201011144424770245".parse::<BigInt>().unwrap(), true)), "354201011144424770245".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"465149022676889812178".parse::<BigInt>().unwrap(), true)), "465149022676889812178".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"622846428783052376944".parse::<BigInt>().unwrap(), true)), "622846428783052376944".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"947868574798565083966".parse::<BigInt>().unwrap(), true)), "947868574798565083966".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"81209374038916873445211".parse::<BigInt>().unwrap(), true)), "81209374038916873445211".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"89227141463320744547444".parse::<BigInt>().unwrap(), true)), "89227141463320744547444".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"304245276624767340763".parse::<BigInt>().unwrap(), true)), "304245276624767340763".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"772766335330795194810".parse::<BigInt>().unwrap(), true)), "772766335330795194810".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"8715984247184847102629".parse::<BigInt>().unwrap(), true)), "8715984247184847102629".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"317398389294290667043".parse::<BigInt>().unwrap(), true)), "317398389294290667043".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"4211495454540623438602".parse::<BigInt>().unwrap(), true)), "4211495454540623438602".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"283880381614019685135".parse::<BigInt>().unwrap(), true)), "283880381614019685135".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"39619211139710770832433".parse::<BigInt>().unwrap(), true)), "39619211139710770832433".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"7185665471565876183582".parse::<BigInt>().unwrap(), true)), "7185665471565876183582".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"4073428941008652389939".parse::<BigInt>().unwrap(), true)), "4073428941008652389939".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"126797938203833874094".parse::<BigInt>().unwrap(), true)), "126797938203833874094".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"161472960118007947140".parse::<BigInt>().unwrap(), true)), "161472960118007947140".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"35090086855192311522542".parse::<BigInt>().unwrap(), true)), "35090086855192311522542".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"8807539481503471142267".parse::<BigInt>().unwrap(), true)), "8807539481503471142267".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"347391647676107808333".parse::<BigInt>().unwrap(), true)), "347391647676107808333".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"742685478636929796464".parse::<BigInt>().unwrap(), true)), "742685478636929796464".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"23501936070330009862657".parse::<BigInt>().unwrap(), true)), "23501936070330009862657".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"666741444048104377839".parse::<BigInt>().unwrap(), true)), "666741444048104377839".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"79913760518204578973905".parse::<BigInt>().unwrap(), true)), "79913760518204578973905".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"72527420513192143286633".parse::<BigInt>().unwrap(), true)), "72527420513192143286633".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"30042727171944117526763".parse::<BigInt>().unwrap(), true)), "30042727171944117526763".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"718671323585233404360".parse::<BigInt>().unwrap(), true)), "718671323585233404360".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"68126089383172504864607".parse::<BigInt>().unwrap(), true)), "68126089383172504864607".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"3059866869120125278993".parse::<BigInt>().unwrap(), true)), "3059866869120125278993".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"62710819416614400313547".parse::<BigInt>().unwrap(), true)), "62710819416614400313547".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"96955472843824997659299".parse::<BigInt>().unwrap(), true)), "96955472843824997659299".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"2282652533088726475824".parse::<BigInt>().unwrap(), true)), "2282652533088726475824".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"724255300584456982182".parse::<BigInt>().unwrap(), true)), "724255300584456982182".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"70597314356773333583940".parse::<BigInt>().unwrap(), true)), "70597314356773333583940".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"1817830065030373843878".parse::<BigInt>().unwrap(), true)), "1817830065030373843878".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"35166875377282288902041".parse::<BigInt>().unwrap(), true)), "35166875377282288902041".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"458456703989975254758".parse::<BigInt>().unwrap(), true)), "458456703989975254758".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"87161122660537307917805".parse::<BigInt>().unwrap(), true)), "87161122660537307917805".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"52947832740072441614366".parse::<BigInt>().unwrap(), true)), "52947832740072441614366".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"1826071513983522132178".parse::<BigInt>().unwrap(), true)), "1826071513983522132178".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"858406154970963776871".parse::<BigInt>().unwrap(), true)), "858406154970963776871".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"23747828523575180599785".parse::<BigInt>().unwrap(), true)), "23747828523575180599785".parse::<BigInt>().unwrap());
}

fn validate_factors(v1: &Vec<BigInt>, v2: &Vec<BigInt>) -> bool {
    if v1.len() != v2.len() {
        return false;
    } else {
        for i in 0..v1.len() {
            if v1[i] != v2[i] {
                return false;
            }
        }
        return true;
    }
}

fn get_square_sum(q: &Quadruple) -> BigInt {
    let mut sum = BigInt::from(0_i32);
    for i in 0..4 {
        sum += &q[i]*&q[i];
    }
    return sum;
}
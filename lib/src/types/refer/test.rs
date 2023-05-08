use {
    crate::types::{
        Keeper,
        Owner,
        Reader,
        RefState,
    },
    std::ops::{
        Deref,
        DerefMut,
    },
};

// explicitly drop all variables to make their lifetime clear
#[test]
fn test_keeper() -> Result<(), RefState> {
    let k1 = Keeper::new("".to_owned());
    assert_ref_state(Keeper::state(&k1), false, 1, 0, false);
    let k2 = Keeper::keeper(&k1)?;
    assert_ref_state(Keeper::state(&k1), false, 2, 0, false);
    {
        let k3 = Keeper::keeper(&k1)?;
        assert_ref_state(Keeper::state(&k1), false, 3, 0, false);
        {
            Keeper::keeper(&k1)?;
            assert_ref_state(Keeper::state(&k1), false, 3, 0, false);
        }
        assert_ref_state(Keeper::state(&k1), false, 3, 0, false);
        drop(k3);
        assert_ref_state(Keeper::state(&k1), false, 2, 0, false);
    }
    assert_ref_state(Keeper::state(&k1), false, 2, 0, false);
    let k4 = Keeper::keeper(&k1)?;
    assert_ref_state(Keeper::state(&k1), false, 3, 0, false);
    let k5 = Keeper::keeper(&k1)?;
    assert_ref_state(Keeper::state(&k1), false, 4, 0, false);
    drop(k4);
    assert_ref_state(Keeper::state(&k1), false, 3, 0, false);
    drop(k2);
    assert_ref_state(Keeper::state(&k1), false, 2, 0, false);
    drop(k5);
    assert_ref_state(Keeper::state(&k1), false, 1, 0, false);
    drop(k1);
    Ok(())
}

// explicitly drop all variables to make their lifetime clear
#[test]
fn test_reader() -> Result<(), RefState> {
    let r1 = Reader::new("".to_owned());
    assert_ref_state(Reader::state(&r1), false, 0, 1, false);
    let r2 = Reader::reader(&r1)?;
    assert_ref_state(Reader::state(&r1), false, 0, 2, false);
    {
        let r3 = Reader::reader(&r1)?;
        assert_ref_state(Reader::state(&r1), false, 0, 3, false);
        {
            Reader::reader(&r1)?;
            assert_ref_state(Reader::state(&r1), false, 0, 3, false);
        }
        assert_ref_state(Reader::state(&r1), false, 0, 3, false);
        drop(r3);
        assert_ref_state(Reader::state(&r1), false, 0, 2, false);
    }
    assert_ref_state(Reader::state(&r1), false, 0, 2, false);
    let r4 = Reader::reader(&r1)?;
    assert_ref_state(Reader::state(&r1), false, 0, 3, false);
    let r5 = Reader::reader(&r1)?;
    assert_ref_state(Reader::state(&r1), false, 0, 4, false);
    drop(r4);
    assert_ref_state(Reader::state(&r1), false, 0, 3, false);
    drop(r2);
    assert_ref_state(Reader::state(&r1), false, 0, 2, false);
    drop(r5);
    assert_ref_state(Reader::state(&r1), false, 0, 1, false);
    drop(r1);
    Ok(())
}

#[test]
fn test_owner() -> Result<(), RefState> {
    let o = Owner::new("".to_owned());
    assert!(Owner::state(&o).is_owned());
    drop(o);
    Ok(())
}

// explicitly drop all variables to make their lifetime clear
#[test]
fn test_mix_ref() -> Result<(), RefState> {
    let k1 = Keeper::new("".to_owned());
    assert_ref_state(Keeper::state(&k1), false, 1, 0, false);
    let r1 = Keeper::reader(&k1)?; // im when box
    assert_ref_state(Keeper::state(&k1), false, 1, 1, false);
    let r2 = Reader::reader(&r1)?; // im when box and im
    assert_ref_state(Keeper::state(&k1), false, 1, 2, false);
    Keeper::keeper(&k1)?; // box when im and box
    assert_ref_state(Keeper::state(&k1), false, 1, 2, false);
    Keeper::owner(&k1).unwrap_err(); // mut when im and box
    assert_ref_state(Keeper::state(&k1), false, 1, 2, false);
    Reader::keeper(&r1)?; // box when im and box
    assert_ref_state(Keeper::state(&k1), false, 1, 2, false);
    drop(r1);
    assert_ref_state(Keeper::state(&k1), false, 1, 1, false);
    Keeper::owner(&k1).unwrap_err(); // mut when box and im
    assert_ref_state(Keeper::state(&k1), false, 1, 1, false);
    drop(r2);
    assert_ref_state(Keeper::state(&k1), false, 1, 0, false);
    let o1 = Keeper::owner(&k1)?; // mut when box
    assert_ref_state(Keeper::state(&k1), false, 1, 0, true);
    Keeper::owner(&k1).unwrap_err(); // mut when box and mut
    assert_ref_state(Keeper::state(&k1), false, 1, 0, true);
    Keeper::reader(&k1).unwrap_err(); // im when box and mut
    assert_ref_state(Keeper::state(&k1), false, 1, 0, true);
    Keeper::keeper(&k1)?; // box when box and mut
    assert_ref_state(Keeper::state(&k1), false, 1, 0, true);
    drop(o1);
    assert_ref_state(Keeper::state(&k1), false, 1, 0, false);
    let r3 = Keeper::reader(&k1)?; // im when box
    assert_ref_state(Keeper::state(&k1), false, 1, 1, false);
    drop(r3);
    assert_ref_state(Keeper::state(&k1), false, 1, 0, false);
    let o2 = Keeper::owner(&k1)?;
    assert_ref_state(Keeper::state(&k1), false, 1, 0, true);
    Owner::drop_data(o2);
    assert_ref_state(Keeper::state(&k1), true, 1, 0, false);
    Keeper::reader(&k1).unwrap_err();
    assert_ref_state(Keeper::state(&k1), true, 1, 0, false);
    Keeper::keeper(&k1)?;
    assert_ref_state(Keeper::state(&k1), true, 1, 0, false);
    Keeper::reinit(&k1, "reinit".to_owned())?;
    assert_ref_state(Keeper::state(&k1), false, 1, 0, false);
    drop(k1);
    Ok(())
}

fn assert_ref_state(
    ref_state: RefState,
    is_dropped: bool,
    keeper_cnt: u32,
    reader_cnt: u32,
    is_owned: bool,
) {
    assert_eq!(ref_state.is_dropped(), is_dropped);
    assert_eq!(ref_state.keeper_cnt(), keeper_cnt);
    assert_eq!(ref_state.reader_cnt(), reader_cnt);
    assert_eq!(ref_state.is_owned(), is_owned);
}

#[test]
fn test_dst() -> Result<(), RefState> {
    let a: Reader<dyn ToString> = Reader::new(1i8);
    let b: Reader<dyn ToString> = Reader::new(true);
    let _ = (a.to_string(), b.to_string());

    let a: Reader<[i8]> = Reader::new([1]);
    let b: Reader<[i8]> = Reader::new([1, 2]);
    let _ = (&a[..], &b[..]);
    Ok(())
}

#[test]
fn test_deref() -> Result<(), RefState> {
    let r1 = Reader::new("".to_owned());
    let r2 = Reader::reader(&r1)?;
    assert_eq!(r1.deref(), "");
    assert_eq!(r2.deref(), "");
    Ok(())
}

#[test]
fn test_deref_mut() -> Result<(), RefState> {
    let mut o = Owner::new("".to_owned());
    assert_eq!(o.deref(), "");
    {
        let s = o.deref_mut();
        s.push('1');
    }
    assert_eq!(o.deref(), "1");
    Ok(())
}

#[test]
fn test_borrow_mut() -> Result<(), RefState> {
    let o = Owner::new("".to_owned());
    assert_eq!(o.deref(), "");
    {
        let s = Owner::borrow_mut(&o);
        s.push('1');
    }
    assert_eq!(o.deref(), "1");
    Ok(())
}

#[test]
fn test_take() -> Result<(), RefState> {
    let o = Owner::new("123".to_owned());
    let k = Owner::keeper(&o)?;
    let s = Owner::move_data(o);
    assert_eq!(s, "123".to_owned());
    Keeper::reader(&k).unwrap_err();
    Keeper::owner(&k).unwrap_err();
    Ok(())
}

#[test]
fn test_reinit() -> Result<(), RefState> {
    let o = Owner::new("123".to_owned());
    let k = Owner::keeper(&o)?;
    Owner::drop_data(o);
    Keeper::reinit(&k, "321".to_owned())?;
    let r = Keeper::reader(&k)?;
    assert_eq!(r.deref(), "321");
    Ok(())
}

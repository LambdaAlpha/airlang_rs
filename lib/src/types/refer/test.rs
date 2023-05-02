use {
    crate::types::{
        CellState,
        Keeper,
        Owner,
        Reader,
    },
    std::ops::{
        Deref,
        DerefMut,
    },
};

// explicitly drop all variables to make their lifetime clear
#[test]
fn test_keeper() -> Result<(), CellState> {
    let b1 = Keeper::new("".to_owned());
    assert_cell_state(Keeper::state(&b1), false, 1, 0, false);
    let b2 = Keeper::keeper(&b1)?;
    assert_cell_state(Keeper::state(&b1), false, 2, 0, false);
    {
        let b3 = Keeper::keeper(&b1)?;
        assert_cell_state(Keeper::state(&b1), false, 3, 0, false);
        {
            Keeper::keeper(&b1)?;
            assert_cell_state(Keeper::state(&b1), false, 3, 0, false);
        }
        assert_cell_state(Keeper::state(&b1), false, 3, 0, false);
        drop(b3);
        assert_cell_state(Keeper::state(&b1), false, 2, 0, false);
    }
    assert_cell_state(Keeper::state(&b1), false, 2, 0, false);
    let b4 = Keeper::keeper(&b1)?;
    assert_cell_state(Keeper::state(&b1), false, 3, 0, false);
    let b5 = Keeper::keeper(&b1)?;
    assert_cell_state(Keeper::state(&b1), false, 4, 0, false);
    drop(b4);
    assert_cell_state(Keeper::state(&b1), false, 3, 0, false);
    drop(b2);
    assert_cell_state(Keeper::state(&b1), false, 2, 0, false);
    drop(b5);
    assert_cell_state(Keeper::state(&b1), false, 1, 0, false);
    drop(b1);
    Ok(())
}

// explicitly drop all variables to make their lifetime clear
#[test]
fn test_reader() -> Result<(), CellState> {
    let i1 = Reader::new("".to_owned());
    assert_cell_state(Reader::state(&i1), false, 0, 1, false);
    let i2 = Reader::reader(&i1)?;
    assert_cell_state(Reader::state(&i1), false, 0, 2, false);
    {
        let i3 = Reader::reader(&i1)?;
        assert_cell_state(Reader::state(&i1), false, 0, 3, false);
        {
            Reader::reader(&i1)?;
            assert_cell_state(Reader::state(&i1), false, 0, 3, false);
        }
        assert_cell_state(Reader::state(&i1), false, 0, 3, false);
        drop(i3);
        assert_cell_state(Reader::state(&i1), false, 0, 2, false);
    }
    assert_cell_state(Reader::state(&i1), false, 0, 2, false);
    let i4 = Reader::reader(&i1)?;
    assert_cell_state(Reader::state(&i1), false, 0, 3, false);
    let i5 = Reader::reader(&i1)?;
    assert_cell_state(Reader::state(&i1), false, 0, 4, false);
    drop(i4);
    assert_cell_state(Reader::state(&i1), false, 0, 3, false);
    drop(i2);
    assert_cell_state(Reader::state(&i1), false, 0, 2, false);
    drop(i5);
    assert_cell_state(Reader::state(&i1), false, 0, 1, false);
    drop(i1);
    Ok(())
}

#[test]
fn test_owner() -> Result<(), CellState> {
    let m = Owner::new("".to_owned());
    assert!(Owner::state(&m).is_mut());
    drop(m);
    Ok(())
}

// explicitly drop all variables to make their lifetime clear
#[test]
fn test_mix_ref() -> Result<(), CellState> {
    let b1 = Keeper::new("".to_owned());
    assert_cell_state(Keeper::state(&b1), false, 1, 0, false);
    let i1 = Keeper::saver(&b1)?; // im when box
    assert_cell_state(Keeper::state(&b1), false, 1, 1, false);
    let i2 = Reader::reader(&i1)?; // im when box and im
    assert_cell_state(Keeper::state(&b1), false, 1, 2, false);
    Keeper::keeper(&b1)?; // box when im and box
    assert_cell_state(Keeper::state(&b1), false, 1, 2, false);
    Keeper::owner(&b1).unwrap_err(); // mut when im and box
    assert_cell_state(Keeper::state(&b1), false, 1, 2, false);
    Reader::keeper(&i1)?; // box when im and box
    assert_cell_state(Keeper::state(&b1), false, 1, 2, false);
    drop(i1);
    assert_cell_state(Keeper::state(&b1), false, 1, 1, false);
    Keeper::owner(&b1).unwrap_err(); // mut when box and im
    assert_cell_state(Keeper::state(&b1), false, 1, 1, false);
    drop(i2);
    assert_cell_state(Keeper::state(&b1), false, 1, 0, false);
    let m1 = Keeper::owner(&b1)?; // mut when box
    assert_cell_state(Keeper::state(&b1), false, 1, 0, true);
    Keeper::owner(&b1).unwrap_err(); // mut when box and mut
    assert_cell_state(Keeper::state(&b1), false, 1, 0, true);
    Keeper::saver(&b1).unwrap_err(); // im when box and mut
    assert_cell_state(Keeper::state(&b1), false, 1, 0, true);
    Keeper::keeper(&b1)?; // box when box and mut
    assert_cell_state(Keeper::state(&b1), false, 1, 0, true);
    drop(m1);
    assert_cell_state(Keeper::state(&b1), false, 1, 0, false);
    let i3 = Keeper::saver(&b1)?; // im when box
    assert_cell_state(Keeper::state(&b1), false, 1, 1, false);
    drop(i3);
    assert_cell_state(Keeper::state(&b1), false, 1, 0, false);
    let m2 = Keeper::owner(&b1)?;
    assert_cell_state(Keeper::state(&b1), false, 1, 0, true);
    Owner::drop_data(m2);
    assert_cell_state(Keeper::state(&b1), true, 1, 0, false);
    Keeper::saver(&b1).unwrap_err();
    assert_cell_state(Keeper::state(&b1), true, 1, 0, false);
    Keeper::keeper(&b1)?;
    assert_cell_state(Keeper::state(&b1), true, 1, 0, false);
    drop(b1);
    Ok(())
}

fn assert_cell_state(
    cell_state: CellState,
    is_dropped: bool,
    box_cnt: u32,
    im_cnt: u32,
    is_mut: bool,
) {
    assert_eq!(cell_state.is_dropped(), is_dropped);
    assert_eq!(cell_state.box_cnt(), box_cnt);
    assert_eq!(cell_state.im_cnt(), im_cnt);
    assert_eq!(cell_state.is_mut(), is_mut);
}

#[test]
fn test_dst() -> Result<(), CellState> {
    let a: Reader<dyn ToString> = Reader::new(1i8);
    let b: Reader<dyn ToString> = Reader::new(true);
    let _ = (a.to_string(), b.to_string());

    let a: Reader<[i8]> = Reader::new([1]);
    let b: Reader<[i8]> = Reader::new([1, 2]);
    let _ = (&a[..], &b[..]);
    Ok(())
}

#[test]
fn test_deref() -> Result<(), CellState> {
    let i1 = Reader::new("".to_owned());
    let i2 = Reader::reader(&i1)?;
    assert_eq!(i1.deref(), "");
    assert_eq!(i2.deref(), "");
    Ok(())
}

#[test]
fn test_deref_mut() -> Result<(), CellState> {
    let mut m = Owner::new("".to_owned());
    assert_eq!(m.deref(), "");
    {
        let s = m.deref_mut();
        s.push('1');
    }
    assert_eq!(m.deref(), "1");
    Ok(())
}

#[test]
fn test_borrow_mut() -> Result<(), CellState> {
    let m = Owner::new("".to_owned());
    assert_eq!(m.deref(), "");
    {
        let s = Owner::borrow_mut(&m);
        s.push('1');
    }
    assert_eq!(m.deref(), "1");
    Ok(())
}

#[test]
fn test_take() -> Result<(), CellState> {
    let m = Owner::new("123".to_owned());
    let b = Owner::keeper(&m)?;
    let s = Owner::move_data(m);
    assert_eq!(s, "123".to_owned());
    Keeper::saver(&b).unwrap_err();
    Keeper::owner(&b).unwrap_err();
    Ok(())
}

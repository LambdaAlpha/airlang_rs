use {
    crate::types::{
        BoxRef,
        CellState,
        ImRef,
        MutRef,
    },
    std::ops::{
        Deref,
        DerefMut,
    },
};

// explicitly drop all variables to make their lifetime clear
#[test]
fn test_box_ref() -> Result<(), CellState> {
    let b1 = BoxRef::new("".to_owned());
    assert_cell_state(BoxRef::state(&b1), false, 1, 0, false);
    let b2 = BoxRef::ref_box(&b1)?;
    assert_cell_state(BoxRef::state(&b1), false, 2, 0, false);
    {
        let b3 = BoxRef::ref_box(&b1)?;
        assert_cell_state(BoxRef::state(&b1), false, 3, 0, false);
        {
            BoxRef::ref_box(&b1)?;
            assert_cell_state(BoxRef::state(&b1), false, 3, 0, false);
        }
        assert_cell_state(BoxRef::state(&b1), false, 3, 0, false);
        drop(b3);
        assert_cell_state(BoxRef::state(&b1), false, 2, 0, false);
    }
    assert_cell_state(BoxRef::state(&b1), false, 2, 0, false);
    let b4 = BoxRef::ref_box(&b1)?;
    assert_cell_state(BoxRef::state(&b1), false, 3, 0, false);
    let b5 = BoxRef::ref_box(&b1)?;
    assert_cell_state(BoxRef::state(&b1), false, 4, 0, false);
    drop(b4);
    assert_cell_state(BoxRef::state(&b1), false, 3, 0, false);
    drop(b2);
    assert_cell_state(BoxRef::state(&b1), false, 2, 0, false);
    drop(b5);
    assert_cell_state(BoxRef::state(&b1), false, 1, 0, false);
    drop(b1);
    Ok(())
}

// explicitly drop all variables to make their lifetime clear
#[test]
fn test_im_ref() -> Result<(), CellState> {
    let i1 = ImRef::new("".to_owned());
    assert_cell_state(ImRef::state(&i1), false, 0, 1, false);
    let i2 = ImRef::ref_im(&i1)?;
    assert_cell_state(ImRef::state(&i1), false, 0, 2, false);
    {
        let i3 = ImRef::ref_im(&i1)?;
        assert_cell_state(ImRef::state(&i1), false, 0, 3, false);
        {
            ImRef::ref_im(&i1)?;
            assert_cell_state(ImRef::state(&i1), false, 0, 3, false);
        }
        assert_cell_state(ImRef::state(&i1), false, 0, 3, false);
        drop(i3);
        assert_cell_state(ImRef::state(&i1), false, 0, 2, false);
    }
    assert_cell_state(ImRef::state(&i1), false, 0, 2, false);
    let i4 = ImRef::ref_im(&i1)?;
    assert_cell_state(ImRef::state(&i1), false, 0, 3, false);
    let i5 = ImRef::ref_im(&i1)?;
    assert_cell_state(ImRef::state(&i1), false, 0, 4, false);
    drop(i4);
    assert_cell_state(ImRef::state(&i1), false, 0, 3, false);
    drop(i2);
    assert_cell_state(ImRef::state(&i1), false, 0, 2, false);
    drop(i5);
    assert_cell_state(ImRef::state(&i1), false, 0, 1, false);
    drop(i1);
    Ok(())
}

#[test]
fn test_mut_ref() -> Result<(), CellState> {
    let m = MutRef::new("".to_owned());
    assert!(MutRef::state(&m).is_mut());
    drop(m);
    Ok(())
}

// explicitly drop all variables to make their lifetime clear
#[test]
fn test_mix_ref() -> Result<(), CellState> {
    let b1 = BoxRef::new("".to_owned());
    assert_cell_state(BoxRef::state(&b1), false, 1, 0, false);
    let i1 = BoxRef::ref_im(&b1)?; // im when box
    assert_cell_state(BoxRef::state(&b1), false, 1, 1, false);
    let i2 = ImRef::ref_im(&i1)?; // im when box and im
    assert_cell_state(BoxRef::state(&b1), false, 1, 2, false);
    BoxRef::ref_box(&b1)?; // box when im and box
    assert_cell_state(BoxRef::state(&b1), false, 1, 2, false);
    BoxRef::ref_mut(&b1).unwrap_err(); // mut when im and box
    assert_cell_state(BoxRef::state(&b1), false, 1, 2, false);
    ImRef::ref_box(&i1)?; // box when im and box
    assert_cell_state(BoxRef::state(&b1), false, 1, 2, false);
    drop(i1);
    assert_cell_state(BoxRef::state(&b1), false, 1, 1, false);
    BoxRef::ref_mut(&b1).unwrap_err(); // mut when box and im
    assert_cell_state(BoxRef::state(&b1), false, 1, 1, false);
    drop(i2);
    assert_cell_state(BoxRef::state(&b1), false, 1, 0, false);
    let m1 = BoxRef::ref_mut(&b1)?; // mut when box
    assert_cell_state(BoxRef::state(&b1), false, 1, 0, true);
    BoxRef::ref_mut(&b1).unwrap_err(); // mut when box and mut
    assert_cell_state(BoxRef::state(&b1), false, 1, 0, true);
    BoxRef::ref_im(&b1).unwrap_err(); // im when box and mut
    assert_cell_state(BoxRef::state(&b1), false, 1, 0, true);
    BoxRef::ref_box(&b1)?; // box when box and mut
    assert_cell_state(BoxRef::state(&b1), false, 1, 0, true);
    drop(m1);
    assert_cell_state(BoxRef::state(&b1), false, 1, 0, false);
    let i3 = BoxRef::ref_im(&b1)?; // im when box
    assert_cell_state(BoxRef::state(&b1), false, 1, 1, false);
    drop(i3);
    assert_cell_state(BoxRef::state(&b1), false, 1, 0, false);
    let m2 = BoxRef::ref_mut(&b1)?;
    assert_cell_state(BoxRef::state(&b1), false, 1, 0, true);
    MutRef::drop_data(m2);
    assert_cell_state(BoxRef::state(&b1), true, 1, 0, false);
    BoxRef::ref_im(&b1).unwrap_err();
    assert_cell_state(BoxRef::state(&b1), true, 1, 0, false);
    BoxRef::ref_box(&b1)?;
    assert_cell_state(BoxRef::state(&b1), true, 1, 0, false);
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
    let a: ImRef<dyn ToString> = ImRef::new(1i8);
    let b: ImRef<dyn ToString> = ImRef::new(true);
    let _ = (a.to_string(), b.to_string());

    let a: ImRef<[i8]> = ImRef::new([1]);
    let b: ImRef<[i8]> = ImRef::new([1, 2]);
    let _ = (&a[..], &b[..]);
    Ok(())
}

#[test]
fn test_deref() -> Result<(), CellState> {
    let i1 = ImRef::new("".to_owned());
    let i2 = ImRef::ref_im(&i1)?;
    assert_eq!(i1.deref(), "");
    assert_eq!(i2.deref(), "");
    Ok(())
}

#[test]
fn test_deref_mut() -> Result<(), CellState> {
    let mut m = MutRef::new("".to_owned());
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
    let m = MutRef::new("".to_owned());
    assert_eq!(m.deref(), "");
    {
        let s = MutRef::borrow_mut(&m);
        s.push('1');
    }
    assert_eq!(m.deref(), "1");
    Ok(())
}

#[test]
fn test_take() -> Result<(), CellState> {
    let m = MutRef::new("123".to_owned());
    let b = MutRef::ref_box(&m)?;
    let s = MutRef::move_data(m);
    assert_eq!(s, "123".to_owned());
    BoxRef::ref_im(&b).unwrap_err();
    BoxRef::ref_mut(&b).unwrap_err();
    Ok(())
}

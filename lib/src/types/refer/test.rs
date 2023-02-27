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
    assert_cell_state(b1.state(), false, 1, 0, false);
    let b2 = b1.ref_box()?;
    assert_cell_state(b1.state(), false, 2, 0, false);
    {
        let b3 = b1.ref_box()?;
        assert_cell_state(b1.state(), false, 3, 0, false);
        {
            b1.ref_box()?;
            assert_cell_state(b1.state(), false, 3, 0, false);
        }
        assert_cell_state(b1.state(), false, 3, 0, false);
        drop(b3);
        assert_cell_state(b1.state(), false, 2, 0, false);
    }
    assert_cell_state(b1.state(), false, 2, 0, false);
    let b4 = b1.ref_box()?;
    assert_cell_state(b1.state(), false, 3, 0, false);
    let b5 = b1.ref_box()?;
    assert_cell_state(b1.state(), false, 4, 0, false);
    drop(b4);
    assert_cell_state(b1.state(), false, 3, 0, false);
    drop(b2);
    assert_cell_state(b1.state(), false, 2, 0, false);
    drop(b5);
    assert_cell_state(b1.state(), false, 1, 0, false);
    drop(b1);
    Ok(())
}

// explicitly drop all variables to make their lifetime clear
#[test]
fn test_im_ref() -> Result<(), CellState> {
    let i1 = ImRef::new("".to_owned());
    assert_cell_state(i1.state(), false, 0, 1, false);
    let i2 = i1.ref_im()?;
    assert_cell_state(i1.state(), false, 0, 2, false);
    {
        let i3 = i1.ref_im()?;
        assert_cell_state(i1.state(), false, 0, 3, false);
        {
            i1.ref_im()?;
            assert_cell_state(i1.state(), false, 0, 3, false);
        }
        assert_cell_state(i1.state(), false, 0, 3, false);
        drop(i3);
        assert_cell_state(i1.state(), false, 0, 2, false);
    }
    assert_cell_state(i1.state(), false, 0, 2, false);
    let i4 = i1.ref_im()?;
    assert_cell_state(i1.state(), false, 0, 3, false);
    let i5 = i1.ref_im()?;
    assert_cell_state(i1.state(), false, 0, 4, false);
    drop(i4);
    assert_cell_state(i1.state(), false, 0, 3, false);
    drop(i2);
    assert_cell_state(i1.state(), false, 0, 2, false);
    drop(i5);
    assert_cell_state(i1.state(), false, 0, 1, false);
    drop(i1);
    Ok(())
}

#[test]
fn test_mut_ref() -> Result<(), CellState> {
    let m = MutRef::new("".to_owned());
    assert!(m.state().is_mut());
    drop(m);
    Ok(())
}

// explicitly drop all variables to make their lifetime clear
#[test]
fn test_mix_ref() -> Result<(), CellState> {
    let b1 = BoxRef::new("".to_owned());
    assert_cell_state(b1.state(), false, 1, 0, false);
    let i1 = b1.ref_im()?; // im when box
    assert_cell_state(b1.state(), false, 1, 1, false);
    let i2 = i1.ref_im()?; // im when box and im
    assert_cell_state(b1.state(), false, 1, 2, false);
    b1.ref_box()?; // box when im and box
    assert_cell_state(b1.state(), false, 1, 2, false);
    b1.ref_mut().unwrap_err(); // mut when im and box
    assert_cell_state(b1.state(), false, 1, 2, false);
    i1.ref_box()?; // box when im and box
    assert_cell_state(b1.state(), false, 1, 2, false);
    drop(i1);
    assert_cell_state(b1.state(), false, 1, 1, false);
    b1.ref_mut().unwrap_err(); // mut when box and im
    assert_cell_state(b1.state(), false, 1, 1, false);
    drop(i2);
    assert_cell_state(b1.state(), false, 1, 0, false);
    let m1 = b1.ref_mut()?; // mut when box
    assert_cell_state(b1.state(), false, 1, 0, true);
    b1.ref_mut().unwrap_err(); // mut when box and mut
    assert_cell_state(b1.state(), false, 1, 0, true);
    b1.ref_im().unwrap_err(); // im when box and mut
    assert_cell_state(b1.state(), false, 1, 0, true);
    b1.ref_box()?; // box when box and mut
    assert_cell_state(b1.state(), false, 1, 0, true);
    drop(m1);
    assert_cell_state(b1.state(), false, 1, 0, false);
    let i3 = b1.ref_im()?; // im when box
    assert_cell_state(b1.state(), false, 1, 1, false);
    drop(i3);
    assert_cell_state(b1.state(), false, 1, 0, false);
    let m2 = b1.ref_mut()?;
    assert_cell_state(b1.state(), false, 1, 0, true);
    m2.delete();
    assert_cell_state(b1.state(), true, 1, 0, false);
    b1.ref_im().unwrap_err();
    assert_cell_state(b1.state(), true, 1, 0, false);
    b1.ref_box()?;
    assert_cell_state(b1.state(), true, 1, 0, false);
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
fn test_deref() -> Result<(), CellState> {
    let i1 = ImRef::new("".to_owned());
    let i2 = i1.ref_im()?;
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

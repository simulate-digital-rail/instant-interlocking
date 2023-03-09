# track_element

This crate provides types and traits to represent and interact with common track elements. It includes types for:

- Points
- KS Light Signals
- Vacancy Sections

Furthermore, it defines the `Driveway` type around which our interlocking architecture is built. In our model, a
driveway is defined as a set of track elements with target states.

Furthermore, this crate defines a basic CLI control station that is useful for testing purposes.

## Example

```rust
// Create the elements of the driveway
let p1 = Point::new_arc(PointState::Left, "P1".to_string());
let p2 = Point::new_arc(PointState::Left, "P2".to_string());
let s = Signal::new_arc(
    SignalState::default(),
    SupportedSignalStates::default()
        .main(&mut vec![MainSignalState::Hp0, MainSignalState::Ks1]),
    "S".to_string(),
    None,
);

// Define its target state
let ts = DrivewayState::new(
    vec![
        (p1.clone(), PointState::Right),
        (p2.clone(), PointState::Left),
    ],
    vec![(s.clone(), (MainSignalState::Ks1).into())],
    vec![],
);

let mut dw = Driveway::new(Vec::new(), ts, s.clone(), s.clone());
```
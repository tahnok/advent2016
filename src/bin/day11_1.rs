use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub struct Building {
    floors: [Floor; 4],
}

impl Building {
    fn new() -> Building {
        Building {
            floors: [
                Floor::new(),
                Floor::new(),
                Floor::new(),
                Floor::new(),
            ]
        }
    }

    fn valid(&self) -> bool {
        self.floors.iter().all(|floor| floor.valid())
    }

    fn moves(&self) -> Vec<Building> {
        let mut buildings = vec![];
        for (index, floor) in self.floors.iter().enumerate() {
            for chip in floor.chips.iter() {
                let mut new_building = self.clone();
                new_building.floors[index].chips.remove(chip);
                if index < 3 {
                    let mut up_one = new_building.clone();
                    up_one.floors[index + 1].chips.insert(*chip);
                    if up_one.valid() {
                        buildings.push(up_one);
                    }
                }
                if index > 0 {
                    let mut down_one = new_building.clone();
                    down_one.floors[index - 1].chips.insert(*chip);
                    if down_one.valid() {
                        buildings.push(down_one);
                    }
                }
            }
        }
        buildings
    }
}

impl Clone for Building {
    fn clone(&self) -> Building {
        let new_floors = [
            self.floors[0].clone(),
            self.floors[1].clone(),
            self.floors[2].clone(),
            self.floors[3].clone(),
        ];
        Building {
            floors: new_floors,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Floor {
    chips: HashSet<Chip>,
    rtgs: HashSet<RTG>,
}

impl Floor {
    fn new() -> Floor {
        Floor {
            chips: HashSet::new(),
            rtgs: HashSet::new(),
        }
    }

    fn valid(&self) -> bool {
        if self.rtgs.len() == 0 {
            return true
        }
        for chip in self.chips.iter() {
            if !self.rtgs.contains(&RTG { kind: chip.kind }) {
                return false
            }
        }
        true
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Chip {
    kind: &'static str,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct RTG {
    kind: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_floor_is_valid() {
        let floor = Floor::new();
        assert!(floor.valid());
    }

    #[test]
    fn floor_with_just_microchip_is_valid() {
        let hydrogen_chip = Chip { kind: "hydrogen" };

        let mut floor = Floor::new();
        floor.chips.insert(hydrogen_chip);
        assert!(floor.valid());
    }

    #[test]
    fn floor_with_just_rtg_is_valid() {
        let hydrogen_rtg = RTG { kind: "hydrogen" };

        let mut floor = Floor::new();
        floor.rtgs.insert(hydrogen_rtg);
        assert!(floor.valid());
    }

    #[test]
    fn floor_with_paired_rtg_and_chip_is_valid() {
        let hydrogen_chip = Chip { kind: "hydrogen" };
        let hydrogen_rtg = RTG { kind: "hydrogen" };

        let mut floor = Floor::new();
        floor.rtgs.insert(hydrogen_rtg);
        floor.chips.insert(hydrogen_chip);

        assert!(floor.valid());
    }

    #[test]
    fn floor_with_unpaired_rtg_and_chip_is_invalid() {
        let hydrogen_chip = Chip { kind: "hydrogen" };
        let lithium_rtg = RTG { kind: "lithium" };

        let mut floor = Floor::new();
        floor.rtgs.insert(lithium_rtg);
        floor.chips.insert(hydrogen_chip);

        assert!(!floor.valid());
    }

    #[test]
    fn empty_building_is_valid() {
        let building = Building::new();
        assert!(building.valid());
    }

    #[test]
    fn building_with_just_chips_is_valid() {
        let mut building = Building::new();

        building.floors[0].chips.insert(Chip{ kind: "hydrogen" });
        building.floors[3].chips.insert(Chip{ kind: "Lithium" });

        assert!(building.valid());
    }

    #[test]
    fn building_with_just_rtgs_is_valid() {
        let mut building = Building::new();

        building.floors[1].rtgs.insert(RTG{ kind: "hydrogen" });
        building.floors[2].rtgs.insert(RTG{ kind: "Lithium" });

        assert!(building.valid());
    }

    #[test]
    fn building_with_paired_chips_and_rtgs_is_valid() {
        let mut building = Building::new();

        building.floors[0].chips.insert(Chip{ kind: "hydrogen" });
        building.floors[0].rtgs.insert(RTG{ kind: "hydrogen" });

        building.floors[0].chips.insert(Chip{ kind: "Lithium" });
        building.floors[0].rtgs.insert(RTG{ kind: "Lithium" });

        assert!(building.valid());
    }

    #[test]
    fn building_with_unpaired_chips_and_rtgs_on_different_floors_is_valid() {
        let mut building = Building::new();

        building.floors[0].chips.insert(Chip{ kind: "hydrogen" });
        building.floors[1].rtgs.insert(RTG{ kind: "hydrogen" });

        building.floors[2].chips.insert(Chip{ kind: "Lithium" });
        building.floors[3].rtgs.insert(RTG{ kind: "Lithium" });

        assert!(building.valid());
    }

    #[test]
    fn building_with_unpaired_chips_and_rtgs_on_same_floor_is_invalid() {
        let mut building = Building::new();

        building.floors[0].chips.insert(Chip{ kind: "Lithium" });
        building.floors[0].rtgs.insert(RTG{ kind: "hydrogen" });

        assert!(!building.valid());
    }

    #[test]
    fn generate_possible_moves_from_first_floor() {
        let mut input = Building::new();
        input.floors[0].chips.insert(Chip{ kind: "Lithium" });

        let mut output = Building::new();
        output.floors[1].chips.insert(Chip{ kind: "Lithium" });

        assert_eq!(vec![output], input.moves());
    }

    #[test]
    fn generate_possible_moves_from_fourth_floor() {
        let mut input = Building::new();
        input.floors[3].chips.insert(Chip{ kind: "Lithium" });

        let mut output = Building::new();
        output.floors[2].chips.insert(Chip{ kind: "Lithium" });

        assert_eq!(vec![output], input.moves());
    }
}

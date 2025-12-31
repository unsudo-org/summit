        cmp::Page {
            scroll_snap: cmp::PageScrollSnap::Proximity,
            bg: conf.color.raisin_black.to_owned(),
            cmp::PageItem {
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "space-between",
                    align_items: "start",
                    min_width: "100vw",
                    max_width: "100vw",
                    min_height: "100vh",
                    max_height: "100vh",
                    div {
                        display: "flex",
                        flex_direction: "row",
                        
                        cmp::NavbarBuild {}
                    }
                    div {
                        min_height: "32px"
                    }
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "center",
                        align_items: "center",
                        min_width: "100%",
                        max_width: "100%",
                        Banner {}
                    }
                    div {
                        flex: "1"
                    }
                    div {
                        flex: "1"
                    }
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "center",
                        align_items: "center",
                        min_width: "100%",
                        max_width: "100%",
                        div {
                            class: "soft_flicker",
                            display: "flex",
                            flex_direction: "column",
                            justify_content: "center",
                            align_items: "center",
                            font_size: "32px",
                            font_family: conf.font.br_cobane,
                            font_weight: "normal",
                            color: conf.color.timberwolf.to_string(),
                            "↡"
                        }
                    }
                    div {
                        min_height: "8px"
                    }
                }
            }
            cmp::PageItem {
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "space-between",
                    align_items: "start",
                    min_width: "100vw",
                    max_width: "100vw",
                    min_height: "100vh",
                    max_height: "100vh",
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "space-between",
                        align_items: "center",
                        min_width: "100%",
                        max_width: "100%",
                        min_height: "100%",
                        max_height: "100%",
                        padding: "16px",
                        transition: "transform 1s",
                        ProblemCard {}
                        ProblemCard {}
                        ProblemCard {}
                    }
                    cmp::HazardStripe {
                        min_w: "100vw",
                        max_w: "100vw",
                        min_h: "32px",
                        max_h: "32px",
                        color_0: conf.color.rose_pompadour.to_owned(),
                        color_1: conf.color.rose_pompadour.to_owned(),
                        color_2: conf.color.raisin_black.to_owned(),
                        color_3: conf.color.raisin_black.to_owned(),
                        size_0: 0,
                        size_1: 20,
                        size_2: 0,
                        size_3: 32,
                        animation_speed_seconds: 64
                    }
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "space-between",
                        align_items: "center",
                        min_width: "100%",
                        max_width: "100%",
                        padding: "16px",
                        div {
                            display: "flex",
                            flex: "1",
                            h4 {
                                display: "flex",
                                min_width: "500px",
                                max_width: "500px",
                                font_family: conf.font.brulia_test,
                                font_weight: "normal",
                                color: conf.color.timberwolf.to_string(),
                                "Web3 is in trouble, our digital town squares are being attacked. We need to do better... We are weakened by red tape and corruption."
                            }
                        }
                        cmp::Shape {
                            w: "258px",
                            h: "258px",
                            fill: conf.color.timberwolf.to_owned(),
                            model: cmp::ShapeModel::AngularStar
                        }
                        div {
                            flex: "1"
                        }
                    }
                    cmp::HazardStripe {
                        min_w: "100vw",
                        max_w: "100vw",
                        min_h: "32px",
                        max_h: "32px",
                        color_0: conf.color.rose_pompadour.to_owned(),
                        color_1: conf.color.rose_pompadour.to_owned(),
                        color_2: conf.color.raisin_black.to_owned(),
                        color_3: conf.color.raisin_black.to_owned(),
                        size_0: 0,
                        size_1: 20,
                        size_2: 0,
                        size_3: 32,
                        animation_speed_seconds: 64
                    }
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "space-between",
                        align_items: "center",
                        min_width: "100%",
                        max_width: "100%",
                        min_height: "100%",
                        max_height: "100%",
                        padding: "16px",
                        transition: "transform 1s",
                        ProblemCard {}
                        ProblemCard {}
                        ProblemCard {}
                    }
                }
            }
            cmp::PageItem {
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "start",
                    padding: "16px",
                    gap: "32px",
                    min_width: "100vw",
                    max_width: "100vw",
                    min_height: "100vh",
                    max_height: "100vh",
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "center",
                        align_items: "center",
                        min_width: "100%",
                        techy_slice::TechySlice {
                            label: "Council",
                            div {
                                display: "flex",
                                flex_direction: "row",
                                flex_wrap: "wrap",
                                justify_content: "center",
                                align_items: "start",
                                min_width: "100%",
                                gap: "16px",
                                CouncilMemberCard {
                                    model: cmp::ShapeModel::FivePointCircleGrid,
                                    name: "PascalCase",
                                    tags: vec!(
                                        "Chief Technology Officer".to_owned()
                                    )
                                }
                                CouncilMemberCard {
                                    model: cmp::ShapeModel::RoundedSquareCross,
                                    name: "JZA",
                                    tags: vec!(
                                        "Principal Product Manager".to_owned()
                                    )
                                }
                                CouncilMemberCard {
                                    model: cmp::ShapeModel::FourLobedFlower,
                                    name: "Frosty",
                                    tags: vec!(
                                        "Senior Project Manager".to_owned()
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }
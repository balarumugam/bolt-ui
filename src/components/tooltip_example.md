pub fn example_component() -> Element {
    rsx!(div {
        style = "margin:32px;",
        class = "example flex flex-center gap-4",
        // Top tooltip
        tooltip("This is a top tooltip", TooltipPosition::Top, 
            rsx!(button {
                "Hover me (Top)"
            })
        ),
        // Right tooltip
        tooltip("This appears on the right", TooltipPosition::Right, 
            rsx!(span {
                "Hover me (Right)"
            })
        ),
        // Bottom tooltip
        tooltip("Bottom tooltip example", TooltipPosition::Bottom, 
            rsx!(div {
                 "Hover me (Bottom)"
            })
        ),
        // Left tooltip
        tooltip("Left side tooltip", TooltipPosition::Left, 
            rsx!(button {
                "Hover me (Left)"
            })
        )
    })
}


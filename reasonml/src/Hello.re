[@react.component]
let make = (~name) =>
    <button> {ReasonReact.string({j| Hello, $name!|j})} </button>

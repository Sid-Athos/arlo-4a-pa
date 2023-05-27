import {createSignal} from "solid-js";

export default function game(){
    const [svg,setSvg] = createSignal("")

    function jsonToSvg(svgFields){
        if(!svgFields.width || !svgFields.height || !svgFields.content || !svgFields.content[0].content){
            return alert("Cannot generate SVG");
        }
        let svg = `<svg width=${svgFields.width} height=${svgFields.height} onClick=test(event)>`;
        svgFields.content.forEach(item => {
            switch (item.tag){
                case "style":
                    svg += `<style>
                                ${item.content}
                            </style>`
                    break;
                case "line":
                    svg += `<line
                                x1=${item.x1} y1=${item.y1}
                                x2=${item.x2}  y2=${item.y2}
                            />`
                    break;
                case "circle":
                    svg += `<circle
                                cx=${item.cx} cy=${item.cy} r=${item.r} fill="#2c3e50"
                            />`
                    break;
                default:
                    console.error("Illegal SVG operation")
            }
        })
        svg += `</svg>`
        setSvg(svg)
    }

    return (
      <>
          {svg}
      </>
    );
}
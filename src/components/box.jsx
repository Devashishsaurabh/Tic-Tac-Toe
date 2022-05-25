import React from 'react'

const style = {
	border: "3px solid red",
	fontSize: "40px",
}   


/*
Box.jsx is the discrete unit structure of the board ui.
It is made a button.
When its pressed it will call the function passed from porps.
*/
const Box = (props) => <button name={props.name} style={style} onClick={props.onClick}> {props.value} </button>

export default Box

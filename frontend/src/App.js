import React from "react";
import './App.css';
class App extends React.Component {
   
    // Constructor 
    constructor(props) {
        super(props);
   
        this.state = {
            items: [],
            DataisLoaded: false
        };
    }
   
    // ComponentDidMount is used to
    // execute the code 
    componentDidMount() {
        fetch(
"http://127.0.0.1:8080/articles")
            .then((res) => res.json())
            .then((json) => {
                this.setState({
                    items: json,
                    DataisLoaded: true
                });
            })
    }
    render() {
        const { DataisLoaded, items } = this.state;
        if (!DataisLoaded) return <div>
            <h1> Please wait some time.... </h1> </div> ;
   
        return (
        <div className = "App">
            <h1> Recent papers in bioinformatics </h1>  {
                items.map((item) => ( 
                <ol key = { item.doi } >
                    User_Name: { item.source }, 
                    Full_Name: { <a href={"http://127.0.0.1:8080/articles/"+item._id}>link text</a> }
                    </ol>
                ))
            }
        </div>
    );
}
}
   
export default App;
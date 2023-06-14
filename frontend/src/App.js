import React from "react";
import './style.css';
import ArticleList from "./components/ArticleList";

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
      <h1> Please wait some time.... </h1> </div>;

    return (
      <div className="container">
        <ArticleList items={items} />
      </div>
    );
  }
}

export default App;

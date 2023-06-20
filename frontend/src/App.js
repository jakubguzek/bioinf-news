import React from "react";
import './style.css';
import ArticleList from "./components/ArticleList";
import Footer from "./components/Footer"
import Header from "./components/Header";

import SquareLoader from "react-spinners/SquareLoader"

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
    if (!DataisLoaded) return <div className="loader-container"> 
      <SquareLoader color={"DarkSalmon"} /> 
    </div>;

    return (
      <div className="container">
        <Header />
        <ArticleList items={items} />
        <Footer />
      </div>
    );
  }
}

export default App;

import {React, useEffect, useState} from "react"
 
export default function RandomArticle() {
    const [article, setArticle] = useState([])

    const fetchArticle = () => {
      fetch("http://127.0.0.1:8080/random-article")
        .then(response => {
          return response.json()
        })
        .then(data => {
          setArticle(data)
        })
    }
  
    useEffect(() => {
      fetchArticle()
    }, [])
  
    return (
        <div>
          <h3>{article.title}</h3>
          <p><b>Authors: </b>{article.authors}</p>
        </div >
      );
}
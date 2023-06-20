import React from "react";
import './style.css';
import ArticlesPage from "./pages/ArticlesPage";
import AppLayout from "./components/AppLayout";
import {Routes, Route} from "react-router-dom"
import RandomArticle from "./pages/RandomArticle";

export default function App() {
  return (
    <Routes>
      <Route element={<AppLayout/>}>
      <Route path="/random" element={<RandomArticle />} />
      <Route path="/" element={<ArticlesPage />} />
      </Route>
    </Routes>
  );
}



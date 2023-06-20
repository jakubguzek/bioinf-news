import React from "react"

import ArticleHeading from "./ArticleHeading";
import ArticleBody from "./ArticleBody";

// This component is responsible for rendering the article entry in ArticleList.
export default function Article(props) {
  /* An object containing the metadata of one article. */
  const { item } = props;
  /*
   * Set initial visibility to false
   * This is generally the pattern used to render some components conditionally
   * `isVisible` is a variable which is later used to determine if the 
   * componenet should be rendered or not. `setIsVisible` is a function that is
   * used to change state. That is to change the value of `isVisible`.
   */
  const [isVisible, setIsVisible] = React.useState(false);

  /* Function used to change the value of `isVisible`. */
  function changeVisibility() {
    /*
     * This is so called function callback. We want to use this pattern if the
     * new value of state (here 'isVisible') depends on the old value. React 
     * silently uses the callback function passed to `setIsVisible` to set new
     * value of `isVisible`. `isVisible` is passed as prevIsVisible paremeter
     * and the result is the new state.
     */
    setIsVisible(prevIsVisible => !prevIsVisible)
  }

  /*
   * Return the componenet itself. This JSX expression is transpiled to the 
   * format acceptable by browser after building the project.
   */
  return (
    <div className="article-entry">
      {/*
        * onClick is used to handle what happens when user clicks the given 
        * html element. Here when user clicks button.article-collapsed-button
        * the `changeVisibility` function is executed, thus flipping the value
        * of `isVisible`. Normally the change of a value within component, does
        * not result in re-rendering of the compoenents, but becaus `isVisible`
        * is changed via `setIsVisible` function. the state is changed and 
        * React redraws components after the state change.
        */}
      <button className="article-collapsed-button" onClick={changeVisibility}>
        <ArticleHeading item={item} isVisible={isVisible} />
        <hr />
      </button>
      {/*
        * This is an example of conditional rendering. ArticleBody element below
        * will be render if and only if `isVisible` variable is true. 
        */}
      {isVisible && <ArticleBody key={item.doi} article={item} />}
    </div>
  )
}

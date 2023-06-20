import React from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faGithub } from "@fortawesome/free-brands-svg-icons"

export default function Footer() {
  return (<footer>
    <div><b>Authors:</b> Jakub Guzek, Paulina Kucharewicz, Mateusz Sikorski</div>
    <div><a className="github-icon" href="https://github.com/jakubguzek/bioinf-news"><FontAwesomeIcon icon={faGithub} /></a></div>
  </footer>
  )
}

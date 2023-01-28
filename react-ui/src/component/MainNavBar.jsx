import React from "react"

export const MainNavBar = () => {
  return (
    <nav className="navbar navbar-expand-lg navbar-dark fixed-top navbar-shrink" id="mainNav">
      <div className="container">
        <a className="navbar-brand js-scroll-trigger" href="#page-top">
          SVG Converter
        </a>
        <button
          className="navbar-toggler navbar-toggler-right"
          type="button"
          data-toggle="collapse"
          data-target="#navbarResponsive"
          aria-controls="navbarResponsive"
          aria-expanded="false"
          aria-label="Toggle navigation"
        >
          Menu
          <i className="fa fa-bars"/>
        </button>
        <div className="collapse navbar-collapse" id="navbarResponsive">
          <ul className="navbar-nav text-uppercase ml-auto">
            <li className="nav-item">
              <a className="nav-link js-scroll-trigger" href="#portfolio">
                Samples
              </a>
            </li>
            <li className="nav-item">
              <a className="nav-link js-scroll-trigger" href="#about">
                About SVG
              </a>
            </li>
          </ul>
        </div>
      </div>
    </nav>
  )
}

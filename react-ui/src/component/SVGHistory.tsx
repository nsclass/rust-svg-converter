import React from "react"

import image1 from "../assets/images/about/1.jpg"
import image2 from "../assets/images/about/2.jpg"
import image3 from "../assets/images/about/3.jpg"
import image4 from "../assets/images/about/4.jpg"

export const SVGHistory = () => {
  return (
    <section id="about">
      <div className="container">
        <div className="row">
          <div className="col-lg-12 text-center">
            <h2 className="section-heading">SVG History from Wikipedia</h2>
            <h3 className="section-subheading text-muted">Scalable Vector Graphics</h3>
            <p className="text-muted">
              Scalable Vector Graphics (SVG) is an XML-based vector image format for two-dimensional
              graphics with support for interactivity and animation. The SVG specification is an
              open standard developed by the World Wide Web Consortium (W3C) since 1999. SVG images
              and their behaviors are defined in XML text files.
            </p>
          </div>
        </div>
        <div className="row">
          <div className="col-lg-12">
            <ul className="timeline">
              <li>
                <div className="timeline-image">
                  <img className="rounded-circle img-fluid" src={image1} alt=""/>
                </div>
                <div className="timeline-panel">
                  <div className="timeline-heading">
                    <h4>September 2001</h4>
                    <h4 className="subheading">Version 1.0</h4>
                  </div>
                  <div className="timeline-body">
                    <p className="text-muted">
                      SVG 1.0 became a W3C Recommendation on 4 September 2001
                    </p>
                  </div>
                </div>
              </li>
              <li className="timeline-inverted">
                <div className="timeline-image">
                  <img className="rounded-circle img-fluid" src={image2} alt=""/>
                </div>
                <div className="timeline-panel">
                  <div className="timeline-heading">
                    <h4>January 2003</h4>
                    <h4 className="subheading">Version 1.1</h4>
                  </div>
                  <div className="timeline-body">
                    <p className="text-muted">
                      SVG 1.1 became a W3C Recommendation on 14 January 2003. The SVG 1.1
                      specification is modularized in order to allow subsets to be defined as
                      profiles. Apart from this, there is very little difference between SVG 1.1
                      and
                      SVG 1.0.
                    </p>
                  </div>
                </div>
              </li>
              <li>
                <div className="timeline-image">
                  <img className="rounded-circle img-fluid" src={image3} alt=""/>
                </div>
                <div className="timeline-panel">
                  <div className="timeline-heading">
                    <h4>December 2008</h4>
                    <h4 className="subheading">Version 1.2</h4>
                  </div>
                  <div className="timeline-body">
                    <p className="text-muted">
                      SVG Tiny 1.2 became a W3C Recommendation on 22 December 2008. It was
                      initially
                      drafted as a profile of the planned SVG Full 1.2 (which has since been
                      dropped
                      in favor of SVG 2), but was later refactored as a standalone specification.
                    </p>
                  </div>
                </div>
              </li>
              <li className="timeline-inverted">
                <div className="timeline-image">
                  <img className="rounded-circle img-fluid" src={image4} alt=""/>
                </div>
                <div className="timeline-panel">
                  <div className="timeline-heading">
                    <h4>August 2011</h4>
                    <h4 className="subheading">Version 1.1 second edition</h4>
                  </div>
                  <div className="timeline-body">
                    <p className="text-muted">
                      SVG 1.1 Second Edition, which includes all the errata and clarifications,
                      but
                      no new features to the original SVG 1.1 was released on 16 August 2011.
                    </p>
                  </div>
                </div>
              </li>
              <li className="timeline-inverted">
                <div className="timeline-image">
                  <h4>Version 2.0 completely rework draft 1.2</h4>
                </div>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </section>
  )
}

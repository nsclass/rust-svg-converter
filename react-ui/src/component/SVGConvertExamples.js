import React from "react"
import svgLogoPNG from "../assets/images/samples/SVG_logo.png"
import svgLogoSVG from "../assets/images/samples/SVG_logo.svg"
import spiderJPG from "../assets/images/samples/spider.jpg"
import spiderSVG from "../assets/images/samples/spider.jpg.svg"

const SVGSample = ({ imageName, originalImage, convertedSvg }) => {
  return (
    <div className="row pb-5 col-lg-12 text-center">
      <div className="col-md-2 col-sm-2"></div>

      <div className="col-md-3 col-sm-3 card">
        <img className="card-img-top" src={originalImage} alt="" />
        <div className="card-body">
          <h5 className="card-title">Orinal image</h5>
          <p className="card-text">{imageName}</p>
        </div>
      </div>

      <div className="col-md-2 col-sm-2 align-self-center">
        <i className="fa  fa-arrow-right fa-5x" aria-hidden="true"></i>
      </div>

      <div className="col-md-3 col-sm-3 card">
        <img className="card-img-top" src={convertedSvg} alt="" />
        <div className="card-body">
          <h5 className="card-title">Converted image</h5>
          <p className="card-text">{imageName}</p>
        </div>
      </div>
      <div className="col-md-2 col-sm-2"></div>
    </div>
  )
}

const SVGConvertExamples = () => {
  return (
    <section className="bg-light" id="portfolio">
      <div className="container">
        <div className="row">
          <div className="col-lg-12 text-center">
            <h2 className="section-heading">SVG Conversion Samples</h2>
            <h3 className="section-subheading text-muted">SVG image conversion samples</h3>
          </div>
        </div>
        <SVGSample imageName="SVG logo" originalImage={svgLogoPNG} convertedSvg={svgLogoSVG} />
        <div className="row"></div>
        <SVGSample imageName="Spider" originalImage={spiderJPG} convertedSvg={spiderSVG} />
      </div>
    </section>
  )
}

export default SVGConvertExamples

import React from "react"
import { useImageDropZone } from "../hooks/useImageDropZone"
import { SVGHistory } from "./SVGHistory"
import { SVGConvertExamples } from "./SVGConvertExamples"
import { SVGConverter } from "./SVGConverter"
import "../assets/css/svgtitle.css"

export const MainView = () => {
  const [filename, fileContent, ImageDropZone] = useImageDropZone()

  const SVGTitle = () => {
    return <>
      <div className="svg-title-background">
        <h1 className="svg-title">SVG Image Converter</h1>
      </div>
    </>
  }

  const SVGConverterView = () => {
    return (
      <header className="masthead" id="page-top">
        <div className="container">
          <div className="intro-text">
            <SVGTitle />
            <div>
              <h5>(Supported image types are PNG and JFG)</h5>
            </div>
            <ImageDropZone />
            <SVGConverter imageFilename={filename} imageData={fileContent} />
          </div>
        </div>
      </header>
    )
  }

  return (
    <div>
      <SVGConverterView />
      <SVGConvertExamples />
      <SVGHistory />
    </div>
  )
}

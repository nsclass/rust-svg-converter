import React, { useState, useCallback } from "react"
import { useDropzone } from "react-dropzone"

import "../assets/css/dropzone.css"

export const useImageDropZone = () => {
  const [filename, setFilename] = useState("")
  const [fileContent, setFileContent] = useState()
  const [errorMessage, setErrorMessage] = useState()

  const ImageDropZone = () => {
    const loadFile = (file) => {
      try {
        setFilename(file.name)

        const reader = new FileReader()
        reader.onloadend = (evt) => {
          if (evt.target.readyState === FileReader.DONE) {
            const content = evt.target.result
            setFileContent(content)
          }
        }
        reader.readAsDataURL(file)
        setErrorMessage(null)
      } catch (e) {
        console.log(e)
        setErrorMessage(JSON.stringify(e, null, 2))
      }
    }

    const onDrop = useCallback((acceptedFiles) => {
      loadFile(acceptedFiles[0])
    }, [])
    const { getRootProps, getInputProps, isDragActive } = useDropzone({ onDrop })

    return (
      <div className="dropzone-container">
        <div className="dropzone" {...getRootProps()}>
          <input {...getInputProps()} />
          {isDragActive ? (
            <p>Drop the file here ...</p>
          ) : (
            <p>Drag 'n' drop a file here, or click to select a file</p>
          )}
        </div>
      </div>
    )
  }

  return [filename, fileContent, ImageDropZone]
}


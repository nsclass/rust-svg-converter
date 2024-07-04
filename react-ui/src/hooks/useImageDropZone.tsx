import React, { useCallback, useState} from "react"
import {useDropzone} from "react-dropzone"

import "../assets/css/dropzone.css"

export const useImageDropZone = (): [string, string, React.FC<{}>] => {
  const [filename, setFilename] = useState("")
  const [fileContent, setFileContent] = useState("")
  const [, setErrorMessage] = useState<string>()

  const ImageDropZone = () => {
    const loadFile = (file: File) => {
      try {
        setFilename(file.name)

        const reader = new FileReader()
        reader.onloadend = (evt) => {
          if (evt?.target?.readyState === FileReader.DONE) {
            const content = evt.target.result as string
            setFileContent(content)
          }
        }
        reader.readAsDataURL(file)
        setErrorMessage("")
      } catch (e) {
        console.log(e)
        setErrorMessage(JSON.stringify(e, null, 2))
      }
    }

    const onDrop = useCallback((acceptedFiles: File[]) => {
      loadFile(acceptedFiles[0])
    }, [])
    const {getRootProps, getInputProps, isDragActive} = useDropzone({onDrop})

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


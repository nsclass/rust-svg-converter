import React from "react"

import "./assets/css/bootstrap.min.css"
import "./assets/css/styles.css"
import { MainNavBar } from "./component/MainNavBar"
import { MainView } from "./component/MainView"
import { ErrorBoundary } from "./component/ErrorBoundary"

export const App = () => {
  return (
    <ErrorBoundary>
      <MainNavBar/>
      <MainView/>
    </ErrorBoundary>
  )
}


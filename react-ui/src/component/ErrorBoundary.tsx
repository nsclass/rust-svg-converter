import React, {ErrorInfo, ReactNode} from "react"


interface Props {
  children: ReactNode;
}

interface State {
  hasError: boolean;
}

export class ErrorBoundary extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {hasError: false}
  }

  static getDerivedStateFromError(error: State) {
    // Update state so the next render will show the fallback UI.
    return {hasError: true}
  }

  componentDidCatch(error:Error, errorInfo:ErrorInfo) {
    // You can also log the error to an error reporting service
    console.error(error, errorInfo)
  }

  render() {
    if (this.state.hasError) {
      return (
        <div className="row col-lg-12 text-center">
          <h1>Unexpected error has happened! Please reload a page</h1>
        </div>
      )
    }

    return this.props.children
  }
}

import React, { Component } from 'react'
import './footer.styl'


var toGithub = () => {
  window.location = 'https://github.com/acmcarther/catalyst'
}

class AppFooter extends Component {
  render () {
    return (
      <div className='bottom-bar'>
        <span className='left-container'>
          <span className='footer-elem'>Made by Alex McArther</span>
        </span>
        <span className='right-container'>
          <a className='footer-elem github-link' onClick={toGithub}>
            Catalyst on Github
          </a>
        </span>
      </div>
    )
  }
}


export default AppFooter

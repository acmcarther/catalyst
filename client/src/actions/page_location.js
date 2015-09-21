import types from '../constants/action_types'


var PageLocationActions = {
  goToLogIn () { return { type: types.GO_TO_LOG_IN } },
  goToRegistration () { return { type: types.GO_TO_REGISTER } },
  goToHelp () { return { type: types.GO_TO_HELP } },
  goToHome () { return { type: types.GO_TO_HOME } },
  goToRepo () {
    return {
      type: types.GO_TO_REPO,
      repoId
    }
  }
}


export default PageLocationActions

webpackHotUpdate(0,{

/***/ 235:
/***/ function(module, exports, __webpack_require__) {

	eval("/* WEBPACK VAR INJECTION */(function(module) {/* REACT HOT LOADER */ if (true) { (function () { var ReactHotAPI = __webpack_require__(59), RootInstanceProvider = __webpack_require__(67), ReactMount = __webpack_require__(69), React = __webpack_require__(123); module.makeHot = module.hot.data ? module.hot.data.makeHot : ReactHotAPI(function () { return RootInstanceProvider.getRootInstances(ReactMount); }, React); })(); } try { (function () {\n\nvar AppFooter, React, a, div, ref;\n\nReact = __webpack_require__(123);\n\nref = React.DOM, div = ref.div, a = ref.a;\n\nAppFooter = React.createClass({\n  render: function() {\n    return div({}, div({}, 'Made by Alex McArther'), a({\n      className: 'github-link',\n      onClick: function() {\n        return window.location = 'https://github.com/acmcarther/catalyst';\n      }\n    }, 'Catalyst on Github'));\n  }\n});\n\nmodule.exports = React.createFactory(AppFooter);\n\n\n/* REACT HOT LOADER */ }).call(this); } finally { if (true) { (function () { var foundReactClasses = module.hot.data && module.hot.data.foundReactClasses || false; if (module.exports && module.makeHot) { var makeExportsHot = __webpack_require__(231); if (makeExportsHot(module, __webpack_require__(123))) { foundReactClasses = true; } var shouldAcceptModule = true && foundReactClasses; if (shouldAcceptModule) { module.hot.accept(function (err) { if (err) { console.error(\"Cannot not apply hot update to \" + \"app_footer.coffee\" + \": \" + err.message); } }); } } module.hot.dispose(function (data) { data.makeHot = module.makeHot; data.foundReactClasses = foundReactClasses; }); })(); } }\n/* WEBPACK VAR INJECTION */}.call(exports, __webpack_require__(33)(module)))\n\n/*****************\n ** WEBPACK FOOTER\n ** ./src/components/app_footer.coffee\n ** module id = 235\n ** module chunks = 0\n **/\n//# sourceURL=webpack:///./src/components/app_footer.coffee?");

/***/ }

})
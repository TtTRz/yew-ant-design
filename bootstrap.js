import './style/global.scss';
import './style/components/header.scss';
import './style/components/menu.scss';


import("./pkg").then(module => {
  module.run_app();
});
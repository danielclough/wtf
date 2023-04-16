const axios = require("axios");
const { blackmore, bittrex } = require("./index.js")

const blackmoreToken = btoa(
  `${blackmore.blackmore_user}:${blackmore.blackmore_password}`
);
const getRpc = axios.create({
  baseURL: `https://${blackmore.blackmoreHost}/api/`,
  headers: {
    Authorization: `Basic ${blackmoreToken}`,
    "Content-type": "application/json",
    "Access-Control-Allow-Origin": `${blackmore.corsOrigin}`
  }
});

const CryptoJS = require('crypto-js')
// set up vars

const getBittrex = async (slug) => {
  let bittrexApiSecret = bittrex.axios.secret
  let bittrexApiKey = bittrex.axios.key
  let timestamp = new Date().getTime()
  let url = 'https://api.bittrex.com/v3';
  let method = "GET"
  let requestBody = ""
  var contentHash = CryptoJS.SHA512(JSON.stringify()).toString(CryptoJS.enc.Hex);
  let returnValue = {}
  var preSign = [timestamp, url + slug, method, contentHash, ''].join('');
  var signature = CryptoJS.HmacSHA512(preSign, bittrexApiSecret).toString(CryptoJS.enc.Hex);

  returnValue = await axios.get(url + slug, {
    headers: {
      "Api-Key": bittrexApiKey,
      "Api-Timestamp": timestamp,
      "Api-Content-Hash": contentHash,
      "Api-Signature": signature
    },
  }).then(response => {
    return response.data;
  }).catch(err => {
    console.log(err);
  });
  return returnValue
}

module.exports = { getRpc, getBittrex }

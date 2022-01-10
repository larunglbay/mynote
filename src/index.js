import 'regenerator-runtime/runtime'

import { initContract, login, logout } from './utils'

import getConfig from './config'
const { networkId } = getConfig(process.env.NODE_ENV || 'development')

var noteCount = 0;
var activeNote = null;

$('#listed').click(function(e){
  var id = e.target.parentElement.id;
  var color = e.target.parentElement.style.backgroundColor;
  activeNote = id;
  console.log('activeNote = ' + activeNote);
  $('#edit-mode').removeClass('no-display').addClass('display');
  var titleSel = $('#' + id)[0].children[0].innerHTML;
  var bodySel = $('#' + id)[0].children[1].innerHTML;
  $('#title-field').val(titleSel);
  $('#body-field').val(bodySel);
  $('notepad').css('background-color', color);
  $('#title-field').css('background-color', color);
  $('#body-field').css('background-color', color);
})

$('#btn-save').click(function(){
  var title = $('#title-field').val();
  var body = $('#body-field').val();
  console.log('title = ' + title);
  console.log('body = ' + body);
  if (title === '' && body === '') {
    alert ('Please add a title or body to your note.');
    return;
  }
  var color = $('notepad').css('background-color');
  var id = noteCount + 1;
  if (activeNote) {
    $('#' + activeNote)[0].children[0].innerHTML = title;
    $('#' + activeNote)[0].children[1].innerHTML = body;
    $('#' + activeNote)[0].style.backgroundColor = color;
    activeNote = null;
    $('#edit-mode').removeClass('display').addClass('no-display');

    // Call smart contract to save data
    editNote(title, body);
  } else {
    var created = new Date();
    $('#listed').append('<div id="note' + id + '" style="background-color: ' + color + '"><div class="list-title">' + title + '</div> <div class="list-text">' + body + '</div> </div>');
    noteCount++;

    // Call smart contract to save data
    insertNote(title, body);
  };
  $('#title-field').val('');
  $('#body-field').val('');
  $('notepad').css('background-color', 'white');
  $('#title-field').css('background-color', 'white');
  $('#body-field').css('background-color', 'white');

});

async function insertNote(noteName, noteContent) {
  try {
    // make an update call to the smart contract
    await window.contract.insert_note({
      // pass the value that the user entered in the greeting field
      name: noteName,
      _content: noteContent
    })
  } catch (e) {
    alert(
      'Something went wrong! ' +
      'Maybe you need to sign out and back in? ' +
      'Check your browser console for more info.'
    )
    throw e
  } finally {
    // re-enable the form, whether the call succeeded or failed
    // fieldset.disabled = false
  }
}

async function editNote(noteName, noteContent) {
  try {
    // make an update call to the smart contract
    await window.contract.edit_note({
      // pass the value that the user entered in the greeting field
      name: noteName,
      _content: noteContent
    })
  } catch (e) {
    alert(
      'Something went wrong! ' +
      'Maybe you need to sign out and back in? ' +
      'Check your browser console for more info.'
    )
    throw e
  } finally {
    // re-enable the form, whether the call succeeded or failed
    // fieldset.disabled = false
  }
}

async function getNotes() {
  var notes = await contract.get_notes();
  console.log(notes);
  $('#listed').html();
  for (const key in notes) {
  // for(var i = 0; i < notes.length; i++) {
    var note = notes[key];
    $('#listed').append('<div id="' + note.title + '"><div class="list-title">' + note.title + '</div> <div class="list-text">' + note.content + '</div> </div>');
    noteCount++;
  }
}

document.querySelector('#sign-in-button').onclick = login
var signoutBtn = document.querySelector('#sign-out-button');
if(signoutBtn !== undefined && signoutBtn !== null) {
  signoutBtn.onclick = logout;
}

// Display the signed-out-flow container
function signedOutFlow() {
  document.querySelector('#signed-out-flow').style.display = 'block'
}

// Displaying the signed in flow container and fill in account-specific data
function signedInFlow() {
  document.querySelector('#signed-in-flow').style.display = 'block';
  document.querySelector('#sign-out-button').style.display = 'block';

  document.querySelectorAll('[data-behavior=account-id]').forEach(el => {
    el.innerText = window.accountId
  });

  $('#account').html(window.accountId);

  getNotes();
}

// `nearInitPromise` gets called on page load
window.nearInitPromise = initContract()
  .then(() => {
    if (window.walletConnection.isSignedIn()) signedInFlow()
    else signedOutFlow()
  })
  .catch(console.error)
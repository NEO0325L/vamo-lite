const chats=[
 {id:'maria',name:'María Cortés',initials:'MC',status:'en línea',time:'10:42',preview:'Perfecto, nos vemos luego',color:'purple',messages:[['theirs','¿Ya viste el documento?','10:39'],['mine','Sí, lo reviso ahora mismo.','10:40'],['theirs','Perfecto, nos vemos luego','10:42']]},
 {id:'equipo',name:'Equipo Vamo',initials:'EV',status:'5 participantes',time:'09:18',preview:'Luis: La prueba quedó lista',color:'blue',messages:[['theirs','La prueba quedó lista para mañana.','09:17'],['mine','Excelente, gracias.','09:18']]},
 {id:'ana',name:'Ana López',initials:'AL',status:'última vez hoy a las 08:30',time:'Ayer',preview:'Te envío las fotos después',color:'orange',messages:[['theirs','Te envío las fotos después.','Ayer']]},
 {id:'soporte',name:'Soporte técnico',initials:'ST',status:'responde normalmente en minutos',time:'Lun',preview:'Tu solicitud fue recibida',color:'green',messages:[['theirs','Tu solicitud fue recibida.','Lun']]}
];
let activeId='maria';
const $=s=>document.querySelector(s); const list=$('#chatList'); const messages=$('#messageList');
function renderChats(filter=''){list.innerHTML=''; chats.filter(c=>c.name.toLowerCase().includes(filter.toLowerCase())).forEach(c=>{const el=document.createElement('div');el.className='chat-item '+(c.id===activeId?'selected':'');el.dataset.id=c.id;el.innerHTML=`<div class="avatar">${c.initials}</div><div class="chat-copy"><strong>${c.name}</strong><span>${c.preview}</span></div><span class="chat-time">${c.time}</span>`;el.onclick=()=>{activeId=c.id;renderChats($('#chatSearch').value);renderConversation()};list.appendChild(el)});$('#chatCount').textContent=`${chats.length} conversaciones`}
function renderConversation(){const c=chats.find(x=>x.id===activeId);$('#headerAvatar').textContent=c.initials;$('#headerName').textContent=c.name;$('#headerStatus').textContent=c.status;messages.innerHTML='<div class="day-divider">Hoy</div>'+c.messages.map(m=>`<article class="message ${m[0]}"><p>${escapeHtml(m[1])}</p><time>${m[2]} ${m[0]==='mine'?'✓✓':''}</time></article>`).join('');messages.scrollTop=messages.scrollHeight}
function escapeHtml(s){const d=document.createElement('div');d.textContent=s;return d.innerHTML}
$('#composer').onsubmit=e=>{e.preventDefault();const input=$('#messageInput');const text=input.value.trim();if(!text)return;const c=chats.find(x=>x.id===activeId);c.messages.push(['mine',text,new Date().toLocaleTimeString('es-CO',{hour:'2-digit',minute:'2-digit'})]);c.preview=text;input.value='';renderChats($('#chatSearch').value);renderConversation()};
$('#chatSearch').oninput=e=>renderChats(e.target.value);
function setSaving(on){document.body.classList.toggle('saving-mode',on);$('#powerBtn').classList.toggle('active',on);$('#savingCheckbox').checked=on;$('#resourceValue').textContent=on?'Muy bajo · 86 MB':'Bajo · 112 MB'}
$('#powerBtn').onclick=()=>setSaving(!document.body.classList.contains('saving-mode'));$('#settingsBtn').onclick=()=>$('#settingsPanel').hidden=false;$('#closeSettings').onclick=()=>$('#settingsPanel').hidden=true;$('#savingCheckbox').onchange=e=>setSaving(e.target.checked);renderChats();renderConversation();

$('#connectBtn').onclick=async()=>{try{if(window.__TAURI_INTERNALS__){await window.__TAURI_INTERNALS__.invoke('open_whatsapp')}else{window.open('https://web.whatsapp.com','_blank')}}catch(e){console.error(e);window.open('https://web.whatsapp.com','_blank')}};





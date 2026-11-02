import gi,json,ctypes
webkit=ctypes.CDLL("libwebkit2gtk-4.1.so.0")
webkit.webkit_web_view_get_snapshot_finish.argtypes=[ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p]
webkit.webkit_web_view_get_snapshot_finish.restype=ctypes.c_void_p
cairo=ctypes.CDLL("libcairo.so.2")
cairo.cairo_surface_write_to_png.argtypes=[ctypes.c_void_p,ctypes.c_char_p]
cairo.cairo_surface_destroy.argtypes=[ctypes.c_void_p]
from pathlib import Path
gi.require_version('Gtk','3.0');gi.require_version('WebKit2','4.1')
from gi.repository import Gtk,WebKit2,GLib
out=Path('.scratch/kvojps-monitor/validation/38');out.mkdir(parents=True,exist_ok=True)
w=Gtk.Window();w.set_default_size(550,800);v=WebKit2.WebView();w.add(v)
cases=[(i,t,width,font) for i in range(-1,8) for t in ['dark','light'] for width,font in [(360,14),(240,21)]]; results=[]
script="""JSON.stringify((()=>{let p=[...document.querySelectorAll('.provider')],m=p.map(x=>x.querySelector('.meter').getBoundingClientRect().top),tracks=p.map(x=>x.querySelector('.track').getBoundingClientRect().top);return {meters:m,tracks,overflow:document.body.scrollWidth>document.body.clientWidth,text:p.map(x=>x.innerText),subgrid:CSS.supports('grid-template-rows','subgrid')}})())"""
def next_case():
 if not cases:
  (out/'geometry.json').write_text(json.dumps(results,indent=2));Gtk.main_quit();return
 global current
 current=cases.pop(0);i,t,width,font=current
 v.load_uri(f'http://127.0.0.1:1422/.scratch/kvojps-monitor/validation/38/index.html?case={i}&theme={t}&width={width}&font={font}')
def check():
 def done(view,result):
  data=json.loads(view.evaluate_javascript_finish(result).to_string());data['case']=current
  assert len(data['tracks'])==2,data
  assert abs(data['tracks'][0]-data['tracks'][1])<1,data
  assert not data['overflow'],data
  results.append(data)
  def captured(view,result):
   i,t,width,font=current
   surface=webkit.webkit_web_view_get_snapshot_finish(hash(view),hash(result),None)
   cairo.cairo_surface_write_to_png(surface,str(out/f'{i}-{t}-{width}-{font}.png').encode())
   cairo.cairo_surface_destroy(surface);next_case()
  v.get_snapshot(WebKit2.SnapshotRegion.VISIBLE,WebKit2.SnapshotOptions.NONE,None,captured)
 v.evaluate_javascript(script,-1,None,None,None,done);return False
def loaded(view,event):
 if event==WebKit2.LoadEvent.FINISHED:GLib.timeout_add(500,check)
v.connect('load-changed',loaded);w.show_all();next_case();GLib.timeout_add_seconds(50,lambda:Gtk.main_quit());Gtk.main()
print(f'{len(results)} scenarios validated')

assert len(results)==36, f"Incomplete validation: {len(results)}/36"

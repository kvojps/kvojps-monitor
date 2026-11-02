import ctypes
import json
from pathlib import Path
import gi

gi.require_version('Gtk', '3.0')
gi.require_version('WebKit2', '4.1')
from gi.repository import Gtk, WebKit2, GLib

out = Path(__file__).resolve().parent
webkit = ctypes.CDLL('libwebkit2gtk-4.1.so.0')
webkit.webkit_web_view_get_snapshot_finish.argtypes = [ctypes.c_void_p] * 3
webkit.webkit_web_view_get_snapshot_finish.restype = ctypes.c_void_p
cairo = ctypes.CDLL('libcairo.so.2')
cairo.cairo_surface_write_to_png.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
cairo.cairo_surface_destroy.argtypes = [ctypes.c_void_p]
window = Gtk.Window()
window.set_default_size(1500, 180)
view = WebKit2.WebView()
window.add(view)
cases = [(case, theme, width, font) for case in ['one', 'two', 'many', 'long'] for theme in ['dark', 'light'] for width, font in [(1400, 12), (760, 12), (320, 18)]]
results = []
errors = []
script = """JSON.stringify((()=>{
const strip = document.querySelector('.operations'), spans = [...strip.querySelector('.summaries').children];
const rects = spans.map(x=>x.getBoundingClientRect());
const overflow = strip.querySelector('.overflow');
return {texts: spans.map(x=>x.textContent), gaps: rects.slice(1).map((r,i)=>r.left-rects[i].right),
unused: spans.map(x=>x.getBoundingClientRect().width-(()=>{const r=document.createRange();r.selectNodeContents(x);return r.getBoundingClientRect().width})()),
overflow: overflow?.textContent ?? '', outside: strip.scrollWidth>strip.clientWidth || document.documentElement.scrollWidth>document.documentElement.clientWidth,
overlap: !!overflow && rects.at(-1).right>overflow.getBoundingClientRect().left,
announcement: document.querySelector('[role=status]').textContent};})())"""

def next_case():
    if not cases:
        (out / 'geometry.json').write_text(json.dumps(results, indent=2, ensure_ascii=False))
        Gtk.main_quit()
        return
    global current
    current = cases.pop(0)
    case, theme, width, font = current
    view.load_uri(f'http://127.0.0.1:1422/.scratch/kvojps-monitor/validation/39/index.html?case={case}&theme={theme}&width={width}&font={font}')

def check():
    def done(view, result):
        try:
            data = json.loads(view.evaluate_javascript_finish(result).to_string())
            data['case'] = current
            results.append(data)
            assert not data['outside'] and not data['overlap'], data
            assert all(abs(gap - 16) < 1 for gap in data['gaps']), data
            if current[0] == 'two' and current[2] == 1400:
                assert len(data['texts']) == 2 and max(data['unused']) < 2, data
            if current[0] == 'many':
                assert data['overflow'] == ({1400: '+3 Módulos', 760: '+4 Módulos', 320: '+5 Módulos'}[current[2]]), data
            assert 'SENTINEL' not in str(data), data
        except Exception as error:
            errors.append(str(error))
        def captured(view, result):
            surface = webkit.webkit_web_view_get_snapshot_finish(hash(view), hash(result), None)
            cairo.cairo_surface_write_to_png(surface, str(out / ('-'.join(map(str, current)) + '.png')).encode())
            cairo.cairo_surface_destroy(surface)
            def expired(view, result):
                try:
                    text = view.evaluate_javascript_finish(result).to_string()
                    assert 'Bordo em repouso' in text or current[0] in ['many', 'long'], text
                except Exception as error:
                    errors.append(str(error))
                next_case()
            view.evaluate_javascript('window.expire()', -1, None, None, None, None)
            GLib.timeout_add(50, lambda: (view.evaluate_javascript('document.querySelector(".operations").textContent', -1, None, None, None, expired), False)[1])
        view.get_snapshot(WebKit2.SnapshotRegion.VISIBLE, WebKit2.SnapshotOptions.NONE, None, captured)
    view.evaluate_javascript(script, -1, None, None, None, done)
    return False

view.connect('load-changed', lambda view, event: GLib.timeout_add(200, check) if event == WebKit2.LoadEvent.FINISHED else None)
window.show_all()
next_case()
GLib.timeout_add_seconds(50, Gtk.main_quit)
Gtk.main()
assert len(results) == 24, len(results)
assert not errors, '\n'.join(errors)
print(f'{len(results)} scenarios validated')

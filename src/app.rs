use log::*;
use std::f64::consts::PI;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;
use web_sys::CanvasRenderingContext2d;
use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
    state: State,
    canvas: Option<HtmlCanvasElement>,
    canvas_ref: NodeRef,
    ctx: Option<CanvasRenderingContext2d>,
}

pub struct State {
    value: String,
}

#[derive(Debug)]
pub struct Xors {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

pub enum Msg {
    RenderCanvas(Event),
    UpdateValue(String),
}

#[derive(Clone)]
pub enum S {
    Cos,
    Sin,
    None
}

impl std::fmt::Debug for S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            S::Cos => write!(f, "S::Cos"),
            S::Sin => write!(f, "S::Sin"),
            S::None => write!(f, "S::None"),
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            state: State { value: "".into() },
            canvas_ref: NodeRef::default(),
            canvas: None,
            ctx: None,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RenderCanvas(evt) => {
                evt.prevent_default();
                self.visual_hash(self.state.value.clone());
            }
            Msg::UpdateValue(val) => {
                println!("Input: {}", val);
                self.state.value = val;
            }
        }
        false
    }

    fn rendered(&mut self, _first_render: bool) {
        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
        canvas.set_height(0);
        canvas.set_width(400);
        
        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        
        self.canvas = Some(canvas);
        self.ctx = Some(ctx);

        info!("rendered");
    }

    fn view(&self) -> Html {
        html! {
            <div class="font-sans wrapper flex justify-center content-center flex-wrap w-full h-screen bg-gray-900">
                <div class="w-10/12 md:w-2/4 lg:w-1/4">
                    <h1 class="text-center text-3xl font-semibold mb-3 text-gray-700">{ "Visual Hashing" }</h1>
                    <section class="content w-full">
                        <form onsubmit=self.link.callback(|e: Event| Msg::RenderCanvas(e))>
                            <input
                                class="bg-gray-800 shadow rounded border-0 border-gray-700 text-gray-600 p-3 w-full"
                                placeholder="Put your name or a text, then press ENTER"
                                value=&self.state.value
                                oninput=self.link.callback(|e: InputData| Msg::UpdateValue(e.value))
                            />
                        </form>
                        <canvas ref={self.canvas_ref.clone()} class="w-full"/>
                    </section>
                    <footer class="info text-center text-gray-600 mt-3">
                        <p>
                            { "Written by " }
                            <a href="https://github.com/emsifa/visual-hashing-yew" target="_blank" class="text-teal-600 font-semibold">{ "Muhammad Syifa" }</a>
                        </p>
                        <p class="text-sm">
                            { "As implementation of " }
                            <a href="https://yew.rs" target="_blank" class="text-teal-600 font-semibold">{ "Yew (WASM)" }</a>
                            {" version of " }
                            <a href="https://github.com/gungunfebrianza/Visual-Hashing" target="_blank" class="text-teal-600 font-semibold">{ "Visual Hashing" }</a>
                            { " by " }
                            <a href="https://github.com/gungunfebrianza" target="_blank" class="text-teal-600 font-semibold">{ "Gun Gun Febrianza" }</a>
                        </p>
                    </footer>
                </div>
            </div>
        }
    }
}

impl App {
    fn visual_hash(&mut self, text: String) {
        self.stp(text);
    }

    fn rand(&mut self, xors: &mut Xors) -> f64 {
        let t = xors.x ^ (xors.x << 11);
        xors.x = xors.y;
        xors.y = xors.z;
        xors.z = xors.w;
        let w2 = xors.w as u32 >> 19;
        let t2 = t as u32 >> 8;
        xors.w = (xors.w ^ (w2 as i32)) ^ (t ^ (t2 as i32));
        (xors.w as f64 / 4294967296.0) + 0.5
    }

    fn stp(&mut self, text: String) {
        let mut c = [123456789, 362436069, 521288629, 0];
        let canvas = self.canvas.clone().unwrap();
        canvas.set_height(400);
        canvas.set_width(400);
        
        let ctx = self.ctx.clone().unwrap();

        for (b, _) in text.clone().bytes().enumerate() {
            let left = text.bytes().nth(b).unwrap() as i32;
            let right = ((b * 11) % 16) as i32;
            info!("left: {}, right: {}", left, right);
            c[(b + 3) % 4] ^= left << right;
        }
        info!("c: {:?}", c);

        let mut xors = Xors {
            x: c[0],
            y: c[1],
            z: c[2],
            w: c[3],
        };

        for _ in 0..52 {
            self.rand(&mut xors);
        }
        
        let nn = match self.fi(&mut xors) {
            true => 7,
            false => 11
        };
        
        let size = 4620;
        
        ctx.set_global_composite_operation("source-over")
        .expect("Failed to set global composite operation to 'source-over'");
        
        ctx.set_fill_style(&JsValue::from("rgb(26, 32, 44)"));
        ctx.fill_rect(0.0, 0.0, 400.0, 400.0);
        
        ctx.set_global_composite_operation("lighter")
        .expect("Failet to set global composite operation to 'lighter'");

        
        let mut h: [f64; 8] = [0.0; 8];
        h[2] = 0.3 + self.rand(&mut xors) * 0.2;
        h[3] = 0.1 + self.rand(&mut xors) * 0.1;
        h[5] = 1.0 + self.rand(&mut xors) * 4.0;
        h[6] = 1.0 + self.rand(&mut xors);
        h[7] = 1.0 + self.rand(&mut xors);
        h[0] = 0.4 + self.rand(&mut xors) * 0.2;
        for a in 2..8 {
            if self.fi(&mut xors) {
                h[a] *= -1.0;
            }
        }
        
        let mut ki = vec![1, 3, 5, 7, 9, 11];
        let mut gu = vec![0, 0, 2, 4, 6, 8, 10];
        let mut s: [(S, S); 8] = [
            (S::None, S::None),
            (S::None, S::None),
            (S::None, S::None),
            (S::None, S::None),
            (S::None, S::None),
            (S::None, S::None),
            (S::None, S::None),
            (S::None, S::None),
        ];
        let mut q: [f64; 8] = [0.0; 8];
        let pr: f64 = ((1.0 + self.rand(&mut xors) * (nn - 1) as f64) as i32 | 0) as f64 / nn as f64;
        
        for a in 0..2 {
            if self.fi(&mut xors) {
                s[a] = (S::Cos, S::Sin);
                q[a] = self.rg(&mut ki, &mut xors) - pr;
            } else {
                s[a] = (S::Sin, S::Cos);
                q[a] = self.rg(&mut gu, &mut xors) + pr;
            }
        }

        for a in 2..8 {
            let mut b = self.fi(&mut xors);
            b = match (ki.len(), gu.len()) {
                (0, _) => false,
                (_, 0) => true,
                _ => b
            };

            q[a] = match b {
                true => self.rg(&mut ki, &mut xors),
                false => self.rg(&mut gu, &mut xors),
            };
            if self.fi(&mut xors) {
                q[a] *= -1.0;
            }
            if a > 5 {
                b = !b;
            }
            s[a] = match b {
                true => (S::Cos, S::None),
                false => (S::Sin, S::None),
            };
        }
        
        let mut n: Vec<f64> = vec![0.0, 0.0, 0.0];
        let mut p: Vec<[f64; 2]> = vec![];
        
        for a in 0..3 {
            n[a] = match self.fi(&mut xors) {
                true => 1.0,
                false => -1.0
            };
        }
        
        let step = PI * 2.0 / (size as f64) * (nn as f64);
        let mut r = 0.0;
        let mut f = 0;

        while f < size {
            let c1 = self.calc(s[3].0.clone(), r * q[3]);
            let bf = self.calc(s[6].0.clone(), r * q[6] + c1 * h[5]) * n[0];
            let af = 1.0 + bf * h[0];
            let mut df = self.calc(s[7].0.clone(), r * q[7]);
            let mut ef = -1.0 * df;
            df *= (2.0 - af) * n[1];
            ef *= (2.0 - af) * n[2];
            let c2 = self.calc(s[5].0.clone(), r * q[5]);
            let cf = self.calc(s[4].0.clone(), r * q[4] + c2 * h[7]) / 4.0 * h[6] * (af - (1.0 - h[0]));
            let xf = (r * pr + cf).sin() *  af + self.calc(s[0].0.clone(), r * q[0]) * h[2] * df + self.calc(s[1].0.clone(), r * q[1]) * h[3] * ef; 
            let yf = (r * pr + cf).cos() *  af + self.calc(s[0].1.clone(), r * q[0]) * h[2] * df + self.calc(s[1].1.clone(), r * q[1]) * h[3] * ef; 
            p.push([xf * 110.0 + 200.0, yf * 110.0 + 200.0]);
            r += step as f64;
            f += 1;
        }

        ctx.begin_path();
        let mut hx = 0;
        for dx in 0..3 {
            let gh = (self.rand(&mut xors) * 360.0) as i32 | 0;
            hx += 1 + (self.rand(&mut xors) * 3.0) as i32 | 0;
            let ih = 50 + (self.rand(&mut xors) * 20.0) as i32 | 0;
            
            let mut ah = 0;
            while ah < p.len() {
                ctx.begin_path();
                let mut ei: Vec<[f64; 2]> = vec![];
                
                for bi in 0..3 {
                    let ci = p[(ah as i32 + bi * ((dx + 1) * hx)) as usize % p.len()];
                    ei.push(ci);
                    ctx.line_to(ci[0] as f64, ci[1] as f64);
                }   
                
                let mut fa = ei[0][0] * (ei[1][1] - ei[2][1]);
                fa += ei[1][0] * (ei[2][1] - ei[0][1]);
                fa += ei[2][0] * (ei[0][1] - ei[1][1]);
                
                if fa > 45.0 && fa < 8000.0 {
                    let color = format!("hsla({}, {}%, 40%, {})", gh, ih, (55.0 / fa));
                    ctx.set_fill_style(&JsValue::from(color));
                    ctx.fill();
                }
                
                ah += 1;
            }
        }
    }

    fn fi(&mut self, xors: &mut Xors) -> bool {
        self.rand(xors) < 0.5
    }

    fn rg(&mut self, ha: &mut Vec<i32>, xors: &mut Xors) -> f64 {
        let c = self.rand(xors);
        let a = (ha.len() as f64 * c as f64) as i32 | 0;
        let b = ha[a as usize];
        ha[a as usize] = ha[ha.len() - 1];
        ha.pop();
        b as f64
    }

    fn calc(&mut self, s: S, num: f64) -> f64 {
        match s {
            S::Cos => num.cos(),
            S::Sin => num.sin(),
            S::None => 0.0
        }
    }
}
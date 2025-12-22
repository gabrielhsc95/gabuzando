use yew::prelude::*;
use crate::window::WindowProps;

pub fn get_home_windows() -> Vec<WindowProps> {
    vec![
        // Cell 1 + 4: CV (Experience)
        WindowProps {
            title: AttrValue::from("cv/experience"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Quantitative Researcher (Senior Associate)"}</b></div>
                            <div>{"Aug. 2024 – Present"}</div>
                        </div>
                        <p><i>{"MSCI"}</i></p>
                        <p>{"Norman, Oklahoma, United States"}</p>
                        <ul>
                            <li>{"Developed an AI financial analyst agent, focusing on named entity recognition and validation."}</li>
                            <li>{"Participated in research discussions to enhance agent capabilities using various LLM models."}</li>
                            <li>{"Understood client engagement to improve on pain points."}</li>
                        </ul>
                    </div>
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Financial Engineer (Senior Associate)"}</b></div>
                            <div>{"Jan. 2024 - Jul. 2024"}</div>
                        </div>
                        <p><i>{"MSCI"}</i></p>
                        <p>{"Norman, Oklahoma, United States"}</p>
                        <ul>
                            <li>{"Teach Led in Testing, empowering 4 people to implement new features to an automated testing software"}</li>
                            <li>{"Managed the testing side implementation of two new pricing models."}</li>
                        </ul>
                    </div>
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Financial Engineer (Associate)"}</b></div>
                            <div>{"Jan. 2023 - Dec. 2023"}</div>
                        </div>
                        <p><i>{"MSCI"}</i></p>
                        <p>{"Norman, Oklahoma, United States"}</p>
                        <ul>
                            <li>{"Analytical Quality Assurance for financial factor models."}</li>
                            <li>{"Developed and reviewed financial building blocks libraries to facilitate the replication of financial models."}</li>
                        </ul>
                    </div>
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Financial Engineer (Analyst)"}</b></div>
                            <div>{"Jun. 2021 - Dec. 2022"}</div>
                        </div>
                        <p><i>{"MSCI"}</i></p>
                        <p>{"Norman, Oklahoma, United States and Budapest, Hungary"}</p>
                        <ul>
                            <li>{"Built an automated testing software for a financial pricing analytics library, and integrated it to the build pipeline."}</li>
                            <li>{"Developed two financial models to estimate financed emissions in accordance with PCAF."}</li>
                        </ul>
                    </div>
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Financial Engineer (Intern)"}</b></div>
                            <div>{"Sep. 2020 - Jun. 2021"}</div>
                        </div>
                        <p><i>{"MSCI"}</i></p>
                        <p>{"Budapest, Hungary"}</p>
                        <ul>
                            <li>{"Analytical Quality Assurance for a financial pricing analytics library."}</li>
                            <li>{"Engage with developer and research about new features and bugs."}</li>
                        </ul>
                    </div>
                </>
            }]),
            x: 1.0,
            y: 10.0,
            width: 31.6,
            height: 55.6,
            buffer: 20.0,
        },
        // Cell 2 + 3: About (Me)
        WindowProps {
            title: AttrValue::from("about/me"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                    <p>{"I am a software developer, a loving partner, a proud step-dad, and an unapologetic nerd with a dash of delightful weirdness, all fueled by my Brazilian roots. Hailing from "} <a href="https://maps.app.goo.gl/7jXanpdULSnsMbwj7">{"Londrina, Paraná, Brazil"}</a>{", my academic background is in the cosmos. I hold a Masters in Cosmology and Astrophysics."}</p>
                    <br />
                    <p>{"Life took a fascinating turn, leading me to the finance industry. I have navigated roles from Financial Engineer (Analytical Quality Assurance) to my current position as a Quantitative Researcher. Ultimately, I see myself as a tool builder, constantly creating and finding solutions."}</p>
                    <br />
                    <p>{"Beyond the world of finance, I am passionate about learning new things. You will often find me at the movies, happily coding personal projects, or immersed in the satisfying click of LEGO bricks."}</p>
                </>
            }]),
            x: 34.6,
            y: 10.0,
            width: 64.2,
            height: 27.3,
            buffer: 20.0,
        },
        // Cell 5: Project (TicTacToe)
        WindowProps {
            title: AttrValue::from("project/tictactoe"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                    <b><a href="https://github.com/gabrielhsc95/tictactoe" target="_blank">{"tictactoe"}</a></b>
                    <p>{"More than just a Tic Tac Toe, it is binary game engine and Artificial Intelligence."}</p>
                </>
            }]),
            x: 34.6,
            y: 39.3,
            width: 30.6,
            height: 26.3,
            buffer: 20.0,
        },
        // Cell 6: Blog (Best)
        WindowProps {
            title: AttrValue::from("blog/best"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <p>{"list, ordered by most likes, of blog posts."}</p>
            }]),
            x: 67.2,
            y: 39.3,
            width: 31.6,
            height: 26.3,
            buffer: 20.0,
        },
        // Cell 7: Project (Mentor XMercury)
        WindowProps {
            title: AttrValue::from("project/mentor_xmercury"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                    <b><a href="https://github.com/Giovani-Costa/project_xmercury" target="_blank">{"project_xmercury"}</a></b>
                    <p>{"Manage a Tabletop RPG game using a discord bot and streamlit app."}</p>
                </>
            }]),
            x: 1.0,
            y: 67.6,
            width: 31.6,
            height: 27.3,
            buffer: 20.0,
        },
        // Cell 8: Project (Mentor XLunar)
        WindowProps {
            title: AttrValue::from("project/mentor_xlunar"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                    <b><a href="https://github.com/Giovani-Costa/project_xlunar" target="_blank">{"project_xlunar"}</a></b>
                    <p>{"Discord bot to help student for standards exams."}</p>
                </>
            }]),
            x: 34.6,
            y: 67.6,
            width: 30.6,
            height: 27.3,
            buffer: 20.0,
        },
        // Cell 9: Home (Greetings)
        WindowProps {
            title: AttrValue::from("home/greetings"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <p>{"hi"}</p>
            }]),
            x: 67.2,
            y: 67.6,
            width: 31.6,
            height: 27.3,
            buffer: 20.0,
        },
    ]
}
